<template>
    <div class="login-container">
        <h1>Registrarse</h1>
        <form @submit.prevent="handleRegister">
            <input v-model="username" placeholder="Usuario">
            <input type="email" v-model="email" placeholder="Correo electrónico">

            <input type="password" v-model="password" placeholder="Contraseña">
            <input type="password" v-model="confirmPassword" placeholder="Confirmar contraseña">

            <button type="submit">Registrar</button>
            <p v-if="message">{{ message }}</p>
        </form>
    </div>
</template>


<script setup>
import { ref } from "vue";

const username = ref("");
const email = ref("");
const password = ref("");
const confirmPassword = ref("");
const message = ref("");

function validarPassword(pwd) {
    const tieneMayuscula = /[A-Z]/.test(pwd);
    const tieneEspecial = /[!@#$%^&*(),.?":{}|<>]/.test(pwd);
    const tieneLongitud = pwd.length >= 8;

    return tieneMayuscula && tieneEspecial && tieneLongitud;
}

async function handleRegister() {
    message.value = "";

    if (!username.value || !email.value || !password.value || !confirmPassword.value) {
        message.value = "Todos los campos son obligatorios.";
        return;
    }

    if (password.value !== confirmPassword.value) {
        message.value = "Las contraseñas no coinciden.";
        return;
    }
     const usernameRegex = /^[A-Za-z][A-Za-z0-9._]{3,}$/;

    if (!usernameRegex.test(username)) {
        return res.status(400).json({
        message:
            "El usuario debe iniciar con letra, tener al menos 4 caracteres y solo usar letras, números, punto o guion bajo."
        });
    }

    if (!validarPassword(password.value)) {
        message.value = "La contraseña debe tener mínimo 8 caracteres, 1 mayúscula y 1 carácter especial.";
        return;
    }
    const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[A-Za-z]{2,}$/;

    if (!emailRegex.test(email.value)) {
        message.value = "El correo electrónico no es válido.";
        return;
    }


    try {
        const res = await fetch("http://localhost:3000/register", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                username: username.value,
                email: email.value,
                password: password.value,
            }),
        });

        const data = await res.json();
        message.value = data.message;

    } catch (error) {
        message.value = "Error de conexión con el servidor";
    }
}
</script>
