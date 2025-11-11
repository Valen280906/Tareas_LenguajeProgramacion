<template>
  <div class="greetings">
    <h1 class="pink">{{ msg }}</h1>
    <h3>
      Esto es el primer modulo >:D
    </h3>
        <button @click="mostrarSaludo = true" class="saludo-btn">
      Mostrar Saludo
    </button>
    
    <div v-if="mostrarSaludo" class="mensaje-hola">
      <h2>Hi!!!</h2>
      <button @click="mostrarSaludo = false" class="cerrar-btn">
        Cerrar
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

defineProps({
  msg: {
    type: String,
    required: true,
  }
})

const mostrarSaludo = ref(false)
</script>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
  color: #42b883;
}

h3 {
  font-size: 1.2rem;
  color: #666;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

.saludo-btn {
  background-color: #42b883;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  margin: 1rem 0;
}

.saludo-btn:hover {
  background-color: #369870;
}

.mensaje-hola {
  text-align: center;
  margin: 1rem 0;
  padding: 1rem;
  background-color: #f8f9fa;
  border-radius: 5px;
  border: 2px solid #42b883;
}

.mensaje-hola h2 {
  color: #42b883;
  font-size: 2.5rem;
  margin: 0 0 1rem 0;
}


.cerrar-btn {
  background-color: #ce1313ff;
  color: white;
  border: none;
  padding: 5px 10px;
  border-radius: 6px;
  cursor: pointer;
}

.cerrar-btn:hover {
  background-color: #ff3742;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}

.pink {
  color: #ff69b4;
}
</style>