<template>
  <div class="app-container">
    <transition name="fade" mode="out-in">
      <component :is="currentView" :user="loggedUser" @login-success="onLoginSuccess" />
    </transition>

    <div v-if="!loggedUser" class="switch-container">
      <p v-if="activeForm === 'RegisterForm'">
        ¿Ya tienes cuenta?
        <span class="switch-link" @click="activeForm = 'LoginForm'">Inicia sesión</span>
      </p>
      <p v-else>
        ¿No tienes cuenta?
        <span class="switch-link" @click="activeForm = 'RegisterForm'">Regístrate</span>
      </p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import LoginForm from "./components/LoginForm.vue";
import RegisterForm from "./components/RegisterForm.vue";
import WelcomeView from "./components/WelcomeView.vue";
import "./style/main.css";
import "./style/forms.css";

const activeForm = ref("LoginForm");
const loggedUser = ref(null);

onMounted(() => {
  const sessionUser = localStorage.getItem("loggedUser");
  if (sessionUser && sessionUser !== "undefined") {
    loggedUser.value = JSON.parse(sessionUser);
  }
});

const currentView = computed(() => {
  if (loggedUser.value) return WelcomeView;
  return activeForm.value === "LoginForm" ? LoginForm : RegisterForm;
});

function onLoginSuccess(user) {
  loggedUser.value = user;
  localStorage.setItem("loggedUser", JSON.stringify(user));
}
</script>
