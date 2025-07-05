##  Usage

`upspot` lets you upload files to your PC (e.g. from a smartphone) over a local hotspot network.

###  Basic usage
```bash
upspot
```
Starts a local server on port 8080, saves uploaded files to the current directory, and displays a QR code pointing to the upload page.

### Example
```bash
upspot 1.1.1.1 8000
```
Starts a local server on 1.1.1.1:8000, saves uploaded files to the current directory, and displays a QR code pointing to the upload page.
### Upload from smartphone


- Enable **hotspot on your PC** 
- Connect your **smartphone** to it
- Run:

  ```bash
  upspot <PC-hotspot-IP> 8000
  ```
- Scan the QR code from phone to upload files
    
## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT)  
and the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).

The dependencies used in this project are primarily licensed under MIT or Apache-2.0 licenses.

See individual crates for details.
