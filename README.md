# Data Privacy Vault

The **Data Privacy Vault** is a secure and efficient service designed to tokenize and detokenize sensitive data. It ensures data privacy by encrypting and storing sensitive information in a secure backend, while providing tokenized representations for safe usage in applications.

## Features

- **Tokenization**: Convert sensitive data into secure tokens.
- **Detokenization**: Retrieve original data from tokens.
- **Data Encryption**: AES-256 encryption for secure storage.
- **Format Validation**: Validate data against specified formats during tokenization.
- **Role-Based Access Control**: API key-based access control for writers and readers.
- **Redis Backend**: High-performance storage for tokenized data.
- **AWS KMS Integration**: Secure key management using AWS Key Management Service.

## Getting Started

### Prerequisites

- Rust (1.72 or later)
- Docker
- Redis server
- AWS credentials with access to KMS

### Setup Instructions

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/iv4n-ga6l/data-privacy-vault.git
   cd data-privacy-vault
   ```

2. **Set Environment Variables**:
   Create a `.env` file in the root directory and configure the following variables:
   ```env
   REDIS_URL=redis://127.0.0.1/
   KMS_KEY_ID=<your-kms-key-id>
   AWS_ACCESS_KEY_ID=<your-aws-access-key-id>
   AWS_SECRET_ACCESS_KEY=<your-aws-secret-access-key>
   AWS_REGION=<your-aws-region>
   ```

   For local development or test runs without AWS KMS, omit `KMS_KEY_ID` and optionally set:
   ```env
   LOCAL_ENCRYPTION_KEY=<any-stable-32-byte-ish-secret>
   ```
   If `KMS_KEY_ID` is not set, the service uses a local symmetric key instead of calling AWS KMS.

3. **Build and Run the Application**:
   Using Docker:
   ```bash
   docker build -t data-privacy-vault .
   docker run -p 8080:8080 --env-file .env data-privacy-vault
   ```

   Or using Cargo:
   ```bash
   cargo build --release
   ./target/release/data_privacy_vault
   ```

4. **Access the Application**:
   The service will be available at `http://127.0.0.1:8080`.

## API Documentation

### Authentication

The service uses API key-based authentication. Include the `x-api-key` header in your requests with one of the following keys:

- **Writer Key**: `writer-key-123` (for tokenization)
- **Reader Key**: `reader-key-456` (for detokenization)

### Endpoints

#### 1. **Tokenize Data**

**Endpoint**: `POST /tokenize`

**Description**: Tokenizes sensitive data and stores it securely.

**Request Body**:
```json
{
  "id": "<unique-request-id>",
  "data": {
    "field1": "value1",
    "field2": "value2"
  },
  "format": {
    "field1": "string",
    "field2": "string"
  }
}
```

- `id`: A unique identifier for the request.
- `data`: Key-value pairs of sensitive data to be tokenized.
- `format` (optional): Specifies the expected format of the data (e.g., string, number).

**Response**:
```json
{
  "id": "<unique-request-id>",
  "data": {
    "field1": "<token1>",
    "field2": "<token2>"
  }
}
```

#### 2. **Detokenize Data**

**Endpoint**: `POST /detokenize`

**Description**: Retrieves original data from tokens.

**Request Body**:
```json
{
  "id": "<unique-request-id>",
  "data": {
    "field1": "<token1>",
    "field2": "<token2>"
  }
}
```

**Response**:
```json
{
  "id": "<unique-request-id>",
  "data": {
    "field1": {
      "found": true,
      "value": "value1"
    },
    "field2": {
      "found": false,
      "value": ""
    }
  }
}
```
- `found`: Indicates whether the token was found in the database.
- `value`: The original value associated with the token (empty if not found).

## Testing

Run the integration tests using the following command:
```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- [Actix Web](https://actix.rs/)
- [Redis](https://redis.io/)
- [AWS KMS](https://aws.amazon.com/kms/)
- [AES Encryption](https://docs.rs/aes/)
