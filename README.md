# 📸 Rust Screenshot Uploader

This application is a Rust-based tool that captures screenshots for each screen of your device at certain intervals and submits them to a remote server along with the MAC address of the device. It's a fun and convenient way to keep track of what's happening on your screen over time. 🖥️

## 🛠️ Stack

- Rust 🦀
- Node.js 🟩

## 📋 Requirements

Make sure you have the following software installed on your system:

- [Cargo](https://doc.rust-lang.org/cargo/) 📦
- [npm](https://www.npmjs.com/get-npm) 📦

## 🚀 Getting Started

1. Clone this repository to your local machine:

```
git clone https://github.com/spacecowb0y/screenshots.git
```

2. Navigate to the project directory:

```
cd screenshots
```

3. Install the Rust dependencies using Cargo:

```
cargo build
```

4. Install the Node.js dependencies using npm:

```
npm install
```

5. Run the application:

```
cargo run
```

The application will start taking screenshots at regular intervals and upload them to a remote server. Sit back and watch your screen moments come to life! ✨

6. To stop the application, press `Ctrl + C`.

## ⚙️ API Server

To run the companion Node.js API server for handling the screenshot uploads, follow these steps:

1. Open a new terminal window.
2. Navigate to the `api` folder in the root directory of the project:

```
cd api
```

3. Install the required Node.js dependencies:

```
npm install
```

4. Start the API server:

```
npm start
```

The API server will start running on port 3000.

Now, both the Rust application and the API server are up and running, ready to capture and display your screen moments! 🖥️📸

## 🖼️ Screenshots Gallery

To view your uploaded screenshots, access the following URL in your browser:

```
http://localhost:3000/uploads/:id
```

Replace `:id` with the ID of the specific screenshot you want to view. Explore your captured moments and relive the memories! 📸🎉

## 📄 License

This project is licensed under the [MIT License](LICENSE). Feel free to modify, enhance, and share it with others. Let's spread the joy of capturing screens! 🌟

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome! Feel free to open an issue or submit a pull request. Together, let's make this tool even more awesome! 🚀

---

Enjoy capturing and sharing your screen moments with Rust Screenshot Uploader! If you have any questions or need assistance, don't hesitate to reach out. Happy screenshotting! 😄📸