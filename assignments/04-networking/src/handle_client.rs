use std::{
    io::{Read, Write},
    net::TcpStream,
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::{
    db::AspirinEatsDb,
    error::AspirinEatsError,
    food::{Order, OrderRequest},
    http::{HttpRequest, HttpResponse},
};

/// Handle incoming client connections
/// 1. Read the request from the stream
/// 2. Parse the request
/// 3. Handle the request based on the HTTP method
/// 4. Send the appropriate response back to the client
/// 5. Handle errors
pub fn handle_client(mut stream: TcpStream, db: Arc<Mutex<AspirinEatsDb>>) {
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                println!("Connection closed by client");
                return;
            }

            // Acquire a read lock on the database
            let db = match db.lock() {
                Ok(db) => db,
                Err(_) => {
                    let response = HttpResponse::from(AspirinEatsError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to acquire database lock\n",
                    )));
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
            };

            // Process the request
            let request = String::from_utf8_lossy(&buffer[..size]);
            println!("Received request: {}", request);

            // Parse the request line
            let http_request = match HttpRequest::from_str(&request) {
                Ok(req) => req,
                Err(_) => {
                    let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
            };

            if http_request.method == Some(String::from("GET")) {
                get_request(&http_request, &db, &mut stream);
                return;
            }
            if http_request.method == Some(String::from("POST")) {
                post_request(&http_request, &db, &mut stream);
                return;
            }
            if http_request.method == Some(String::from("DELETE")) {
                delete_request(&http_request, &db, &mut stream);
                return;
            } else {
                let response = HttpResponse::from(AspirinEatsError::MethodNotAllowed);
                stream.write_all(response.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        }
        Err(e) => {
            let response = HttpResponse::from(AspirinEatsError::Io(e));
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            false
        }
    } {}
}

/// Handle GET requests
fn get_request(http_request: &HttpRequest, db: &AspirinEatsDb, stream: &mut TcpStream) {
    if let Some(path) = &http_request.path {
        if let Some(id) = path.strip_prefix("/orders") {
            let mut orders = Vec::<Order>::new();
            if id.is_empty() {
                // Query the database for all orders
                orders = match db.get_all_orders() {
                    Ok(orders) => orders,
                    Err(e) => {
                        let response = HttpResponse::from(AspirinEatsError::Database(e));
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
            } else {
                // Handle specific order requests
                let id = match id.strip_prefix("/") {
                    Some(id) => id,
                    None => {
                        let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let id = match id.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                orders = match db.get_order(id) {
                    Ok(Some(order)) => {
                        orders.push(order);
                        orders
                    }
                    Ok(None) => {
                        let response = HttpResponse::from(AspirinEatsError::NotFound);
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                    Err(e) => {
                        let response = HttpResponse::from(AspirinEatsError::Database(e));
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
            }
            // Serialize the orders to JSON
            let response_body = match serde_json::to_string(&orders) {
                Ok(response_body) => format!("{}\n", response_body),
                Err(e) => {
                    let response = HttpResponse::from(AspirinEatsError::ParseError(e));
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
            };
            // Send the response
            let response = HttpResponse::new(200, "OK", &response_body);
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        } else if path == "/" {
            let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!\n");
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        } else {
            // Handle invalid path
            let response = HttpResponse::from(AspirinEatsError::NotFound);
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
    // Handle invalid request
    let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// Handle POST requests
fn post_request(http_request: &HttpRequest, db: &AspirinEatsDb, stream: &mut TcpStream) {
    // Handle POST requests
    if let Some(path) = &http_request.path {
        if path == "/orders" {
            let body = http_request.body.clone().unwrap_or_default();
            // Create a new order
            let order: OrderRequest = match serde_json::from_str(&body) {
                Ok(order) => order,
                Err(e) => {
                    let response = HttpResponse::from(AspirinEatsError::ParseError(e));
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
            };
            let order = Order::from(order);
            match db.add_order(order) {
                Ok(id) => id,
                Err(e) => {
                    let response = HttpResponse::from(AspirinEatsError::Database(e));
                    stream.write_all(response.to_string().as_bytes()).unwrap();
                    stream.flush().unwrap();
                    return;
                }
            };
            let response = HttpResponse::new(200, "OK", "Order created successfully\n");
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        } else if path == "/" {
            let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!");
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
        let response = HttpResponse::from(AspirinEatsError::NotFound);
        stream.write_all(response.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// Handle DELETE requests
fn delete_request(http_request: &HttpRequest, db: &AspirinEatsDb, stream: &mut TcpStream) {
    if let Some(path) = &http_request.path {
        if let Some(id) = path.strip_prefix("/orders") {
            if id.is_empty() {
                // Delete all orders
                match db.reset_orders() {
                    Ok(orders) => orders,
                    Err(e) => {
                        let response = HttpResponse::from(AspirinEatsError::Database(e));
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let response = HttpResponse::new(200, "OK", "All orders deleted successfully\n");
                stream.write_all(response.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            } else {
                // Handle specific order deletions
                let id = match id.strip_prefix("/") {
                    Some(id) => id,
                    None => {
                        let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let id = match id.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                match db.remove_order(id) {
                    Ok(order) => order,
                    Err(e) => {
                        let response = HttpResponse::from(AspirinEatsError::Database(e));
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let response = HttpResponse::new(200, "OK", "Order deleted successfully\n");
                stream.write_all(response.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        } else if path == "/" {
            let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!");
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        } else {
            // Handle invalid path
            let response = HttpResponse::from(AspirinEatsError::NotFound);
            stream.write_all(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
    // Handle invalid request
    let response = HttpResponse::from(AspirinEatsError::InvalidRequest);
    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn setup_test_server() -> (TcpListener, Arc<Mutex<AspirinEatsDb>>) {
        let db = AspirinEatsDb::in_memory().unwrap();
        // Add two test orders
        let order_str = "{\"customer\":\"Amit\",\"food\":[{\"Burger\":{\"bun\":\"Plain\",\"patty\":\"Beef\",\"toppings\":[\"Lettuce\",\"Tomato\",\"Bacon\"]}}, \"Fries\"]}";
        let order: OrderRequest = serde_json::from_str(order_str).unwrap();
        db.add_order(Order::from(order)).unwrap();

        let arc = Arc::new(Mutex::new(db));
        let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // Bind to any available port
        let arc_clone = Arc::clone(&arc);
        let listener_clone = listener.try_clone().unwrap();

        thread::spawn(move || {
            for stream in listener_clone.incoming() {
                match stream {
                    Ok(stream) => {
                        let arc = Arc::clone(&arc_clone);
                        thread::spawn(move || {
                            handle_client(stream, arc);
                        });
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        });

        (listener, arc)
    }

    fn send_request(request: &str, port: u16) -> String {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 512];
        let size = stream.read(&mut buffer).unwrap();
        String::from_utf8_lossy(&buffer[..size]).to_string()
    }

    #[test]
    fn test_get_orders() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "GET /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        let expected_status_code = 200;
        let expected_status_text = "OK";
        let expected_body = "[{\"id\":1,\"customer\":\"Amit\",\"food\":[{\"Burger\":{\"bun\":\"Plain\",\"patty\":\"Beef\",\"toppings\":[\"Lettuce\",\"Tomato\",\"Bacon\"]}},\"Fries\"],\"status\":\"Pending\",\"total\":15.0}]\n";
        let expected_response =
            HttpResponse::new(expected_status_code, expected_status_text, expected_body);

        assert!(response == expected_response.to_string());
    }

    #[test]
    fn test_post_order_and_get_order_by_id() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "POST /orders HTTP/1.1\r\nHost: localhost\r\n\r\n{\"customer\":\"Brooke\",\"food\":[{\"Burger\":{\"bun\":\"Sesame\",\"patty\":\"Veggie\",\"toppings\":[\"Lettuce\",\"Onion\",\"Cheese\"]}}, \"Drink\"]}";
        let response = send_request(request, port);

        assert!(response.contains("200 OK"));
        assert!(response.contains("Order created successfully"));

        let request = "GET /orders/2 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        let expected_status_code = 200;
        let expected_status_text = "OK";
        let expected_body = "[{\"id\":2,\"customer\":\"Brooke\",\"food\":[{\"Burger\":{\"bun\":\"Sesame\",\"patty\":\"Veggie\",\"toppings\":[\"Lettuce\",\"Onion\",\"Cheese\"]}},\"Drink\"],\"status\":\"Pending\",\"total\":11.0}]\n";
        let expected_response =
            HttpResponse::new(expected_status_code, expected_status_text, expected_body);

        assert!(response == expected_response.to_string());
    }

    #[test]
    fn test_delete_order() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "DELETE /orders/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        assert!(response.contains("200 OK"));
        assert!(response.contains("Order deleted successfully"));

        let request = "GET /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        let expected_status_code = 200;
        let expected_status_text = "OK";
        let expected_body = "[]\n";
        let expected_response =
            HttpResponse::new(expected_status_code, expected_status_text, expected_body);

        assert!(response == expected_response.to_string());
    }

    #[test]
    fn test_delete_all_orders() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "DELETE /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        assert!(response.contains("200 OK"));
        assert!(response.contains("All orders deleted successfully"));

        let request = "GET /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        let expected_status_code = 200;
        let expected_status_text = "OK";
        let expected_body = "[]\n";
        let expected_response =
            HttpResponse::new(expected_status_code, expected_status_text, expected_body);

        assert!(response == expected_response.to_string());
    }

    #[test]
    fn test_invalid_path() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "GET /invalid_path HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        assert!(response.contains("404 Not Found"));
    }

    #[test]
    fn test_invalid_method() {
        let (listener, _db) = setup_test_server();
        let port = listener.local_addr().unwrap().port();

        let request = "PUT /orders HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = send_request(request, port);

        assert!(response.contains("405 Method Not Allowed"));
    }
}
