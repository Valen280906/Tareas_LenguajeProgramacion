<template>
    <div v-if="user" class="welcome-container">
        <h1>Bienvenido, {{ user.username }}!</h1>
        <p>Tu correo: {{ user.email }}</p>
        <button @click="logout">Cerrar Sesión</button>
    </div>

</template>

<script setup>
const props = defineProps({
  user: Object
});

const emit = defineEmits(["logout"]);

function logout() {
  localStorage.removeItem("loggedUser");
  emit("logout");
}
</script>
