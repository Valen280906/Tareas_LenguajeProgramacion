<template>
    <div class="login-container">
        <h1>Registrarse</h1>
        <form @submit.prevent="handleRegister">
            <input v-model="username" placeholder="Usuario">
            <input type="email" v-model="email" placeholder="Correo eléctronico">
            <input type="password" v-model="password" placeholder="Contraseña">
             <button type="submit"> Registrar</button>
             <p v-if="message">{{ message }}</p>
        </form>
    </div>
</template>

<script setup>
import {ref} from "vue";

const username = ref("");
const email = ref("");
const password = ref("");
const message = ref("");

async function handleRegister() {
    try{
        const res = await fetch("http://localhost:3000/register",{
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify({
                username: username.value,
                email:email.value,
                password:password.value,
            }),
        });

        const data = await res.json();
        message.value = data.message;
    }catch(error){
        message.value = "Error de conexión con el servidor";
    }
}
</script>