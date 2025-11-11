<template>
  <div>
    <p> Has hecho click {{ numero }} veces</p>
    <button @click="numero++">Incrementar</button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const numero = ref(0)
</script>

<style scoped>
.greetings {
  text-align: center;
  margin: 2rem 0;
}

button {
  background-color: #42b883;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
}

button:hover {
  background-color: #369870;
}

p {
  font-size: 1.2rem;
  color: #2c3e50;
}
</style>