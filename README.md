# Verihubs MCP Server

An MCP Server that connect to [Verihubs](https://verihubs.com) services such as OCR, facial recognition, document scanning, etc.


## Tools list

### `extract_ktp_data`
The server provides `extract_ktp_data` tool that accepts base64 image that contains a KTP (Indonesian ID Card) and returns extracted KTP data in JSON.

**Parameters:**
- `image` (string): base64 image containing KTP ID Card

## Usage

### Running the MCP Server

Start the weather MCP server:

```bash
cargo run
```

### Debugging the MCP Server

Start the weather MCP server with npx `@modelcontextprotocol/inspector:`

```bash
npx @modelcontextprotocol/inspector
```

### Running via Docker

WIP


## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
