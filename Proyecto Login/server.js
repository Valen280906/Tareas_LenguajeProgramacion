import express from "express";
import fs from "fs";
import path from "path";
import cors from "cors";
import bcrypt from "bcrypt";

const app = express();
app.use(cors());
app.use(express.json());

const DATA_PATH = path.resolve("./data.json");

if (!fs.existsSync(DATA_PATH)) {
  fs.writeFileSync(DATA_PATH, JSON.stringify([]));
  console.log("Archivo data.json creado automáticamente");
}

const leerUsuarios = () => JSON.parse(fs.readFileSync(DATA_PATH, "utf8"));
const guardarUsuarios = (usuarios) =>
  fs.writeFileSync(DATA_PATH, JSON.stringify(usuarios, null, 2));

app.post("/register", async (req, res) => {
  const { username, email, password } = req.body;

  if (!username || !email || !password)
    return res.status(400).json({ message: "Todos los campos son obligatorios." });

  const users = leerUsuarios();

  const existe = users.find(
    (u) => u.username === username || u.email === email
  );
  if (existe)
    return res.status(400).json({ message: "Usuario o correo ya registrado." });

  if (password.length < 6)
    return res.status(400).json({ message: "La contraseña debe tener al menos 6 caracteres." });

  const hash = await bcrypt.hash(password, 10);

  users.push({ username, email, password: hash, fecha: new Date().toISOString() });
  guardarUsuarios(users);

  res.json({ message: "Registro exitoso. Ahora puedes iniciar sesión." });
});

app.post("/login", async (req, res) => {
  const { username, password } = req.body;

  if (!username || !password)
    return res.status(400).json({ message: "Todos los campos son obligatorios." });

  const users = leerUsuarios();
  const user = users.find((u) => u.username === username);

  if (!user) return res.status(404).json({ message: "El usuario no existe." });

  const valido = await bcrypt.compare(password, user.password);
  if (!valido)
    return res.status(401).json({ message: "Contraseña incorrecta." });

  res.json({
    message: "Inicio de sesión exitoso",
    user: { username: user.username, email: user.email },
  });
});

app.listen(3000, () => console.log("Servidor en http://localhost:3000"));
