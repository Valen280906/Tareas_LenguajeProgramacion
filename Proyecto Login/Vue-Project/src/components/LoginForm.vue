<template>
  <div class="login-container">
    <h1>Iniciar Sesión</h1>
    <form @submit.prevent="handleLogin">
      <input v-model="username" placeholder="Usuario" />
      <input type="password" v-model="password" placeholder="Contraseña" />

      <div class="remember-container">
        <input type="checkbox" v-model="remember" id="remember" />
        <label for="remember">Recordarme</label>
      </div>

      <button type="submit">Entrar</button>
      <p v-if="message">{{ message }}</p>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted} from "vue";
const emit = defineEmits(["login-success"]);

const username = ref("");
const password = ref("");
const remember = ref(false);
const message = ref("");

onMounted(() => {
  const savedUser = localStorage.getItem("rememberedUser");
  if (savedUser) {
    username.value = savedUser;
    remember.value = true;
  }
});

async function handleLogin() {
  message.value = "";

  if (!username.value || !password.value) {
    message.value = "Por favor completa todos los campos.";
    return;
  }

  try {
    const res = await fetch("http://localhost:3000/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        username: username.value,
        password: password.value,
      }),
    });

    const data = await res.json();
    message.value = data.message;

    if (res.ok) {
      emit("login-success", data.user); 

      if (remember.value) {
        localStorage.setItem("rememberedUser", username.value);
      } else {
        localStorage.removeItem("rememberedUser");
      }
    }
  } catch (error) {
    message.value = "Error de conexión con el servidor.";
    console.error(error);
  }
}
</script>
