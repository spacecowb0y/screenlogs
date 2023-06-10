const fs = require("fs");
const express = require("express");
const multer = require("multer");
const sqlite3 = require("sqlite3");

const app = express();
const upload = multer({ dest: "public/uploads/" });

const InitDB = () => {
  let db = new sqlite3.Database("./db.sqlite3", (err) => {
    if (err) {
      return err.message;
    }
    db.run(
      `CREATE TABLE IF NOT EXISTS images (id INTEGER PRIMARY KEY AUTOINCREMENT, originalname TEXT, filename TEXT, path TEXT, size INTEGER, mimetype TEXT, mac_address TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)`
    );
  });
  return db;
};

app.use(express.static("public"));

app.get("/uploads/:id", (req, res) => {
  let db = new sqlite3.Database("./db.sqlite3", (err) => {
    if (err) {
      return err.message;
    }
    const { id } = req.params;
    const query = `SELECT * FROM images WHERE id = ?`;
    db.get(query, [id], (err, row) => {
      if (err) {
        return err.message;
      }
      if (!row || (row && !fs.existsSync(row.path))) {
        return res
          .status(404)
          .json({ status: 404, message: "Image not found" });
      }

      const { path, mimetype } = row;
      res.set("Content-Type", mimetype).sendFile(path, { root: __dirname });
    });
  });
});

app.post("/upload", upload.array("images"), (req, res) => {
  let insert =
    "INSERT INTO images (originalname, filename, path, size, mimetype, mac_address, created_at) VALUES (?,?,?,?,?,?,?)";
  let db = new sqlite3.Database("./db.sqlite3", (err) => {
    if (err) {
      return err.message;
    }
    req.files.forEach((file) => {
      const { originalname, filename, path, size, mimetype } = file;
      const record = db.run(insert, [
        originalname,
        filename,
        path,
        size,
        mimetype,
        req.body.mac_address,
        new Date(),
      ]);
      if (!record) {
        console.log("Error inserting record");
      }
    });
  });
  res.status(200).json({ status: 200, message: "Files uploaded successfully" });
});

app.listen(3000, () => {
  console.log(InitDB());
  console.log("Server is listening on port 3000");
});
