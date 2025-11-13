<template>
    <div v-if="user" class="welcome-container">
        <h1>Bienvenido, {{ user.username }}!</h1>
        <p>Tu correo: {{ user.email }}</p>
        <button @click="logout">Cerrar Sesión</button>
    </div>

</template>

<script setup>
    import { defineProps } from 'vue';

    const props = defineProps({
        user:Object
    });

    function logout(){
        localStorage.removeItem("loggedUser");
        window.location.reload();
    }
</script>