# Dive 🏊‍♂️

**Dive** is a key-value caching system built in Rust, designed to offer robust security and stability with the performance benefits of Rust. It is a simple and easy-to-use caching solution that supports various advanced features for managing key-value stores efficiently.

## Features 🌟

- **In-Memory Key-Value Store**: Efficiently store and retrieve key-value pairs.
- **Frequency-Based Eviction**: Implements LFU (Least Frequently Used) eviction policy to manage cache size.
- **File Persistence**: Save and load the store from CSV files for persistent storage.
- **Concurrency Support**: Utilize async operations for high concurrency and performance.
- **Basic HTTP API**: Interact with the cache through a RESTful API built with Axum.

## Future Plans 🚀

- **CLI Functionality**: Add command-line interface tools for managing the cache.
- **Additional HTTP Handlers**: Implement more HTTP endpoints for extended functionality.
- **Max Store Size**: Introduce limits on the size of each store.
- **Periodic Snapshots & Write-Ahead Logging**: Ensure data integrity and recovery.
- **Automatic LFU & LRU Eviction**: Improve eviction strategies for better cache management.

## Installation ⚙️

To run Dive locally, follow these steps:

1. **Clone the Repository**:
    
    ```bash
    git clone <https://github.com/yourusername/dive.git>
    cd dive
    
    ```
    
2. **Build and Run**:
    
    Make sure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
    
    ```bash
    cargo build
    cargo run
    
    ```
    
    This will start the server on `http://127.0.0.1:3000`.
    

## API Endpoints 📡

### Create a New Store

- **Endpoint**: `/create-store`
- **Method**: `POST`
- **Description**: Creates a new key-value store instance.
- **Response**: Returns the ID of the newly created store.

### Insert Data into Store

- **Endpoint**: `/insert/:instance_id`
- **Method**: `POST`
- **Description**: Inserts key-value pairs into the specified store instance.
- **Request Body**:
    
    ```json
    {
        "data": [
            ["key1", "value1"],
            ["key2", "value2"]
        ]
    }
    
    ```
    
- **Response**: Status of the insertion operation.

### Retrieve Data from Store

- **Endpoint**: `/get/:id`
- **Method**: `GET`
- **Description**: Retrieves the value for a specified key from the store instance.
- **Response**: Returns the stored value or an error message.

## Usage Example 💻

Here’s an example of how to use the API with `curl`:

- **Create a Store**:
    
    ```bash
    curl -X POST <http://localhost:3000/create-store>
    
    ```
    
- **Insert Data**:
    
    ```bash
    curl -X POST <http://localhost:3000/insert/><instance_id> -H "Content-Type: application/json" -d '{"data":[["key1","value1"]]}'
    
    ```
    
- **Retrieve Data**:
    
    ```bash
    curl -X GET <http://localhost:3000/get/><instance_id>
    
    ```
    

## Contributing 🤝

We welcome contributions to **Dive**! If you have suggestions or improvements, please open an issue or submit a pull request.

- **Fork the Repository**: Make changes in your fork and test them.
- **Submit a Pull Request**: Describe your changes and why they are needed.

## License 📝

This project is licensed under the MIT License. See the [LICENSE](https://www.notion.so/LICENSE) file for details.

## Contact 📧

For any questions or feedback, please reach out to divinewisdom.dev@gmail.com.
