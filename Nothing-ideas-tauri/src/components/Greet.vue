<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <div class="mb-4">
    <el-button>Default</el-button>
    <router-link  to="/One">我是1</router-link>
    <router-link  to="/Two">我是2</router-link>
    <el-button type="primary" @Click="$router.push('/One')">One</el-button>
    <el-button type="success" @Click="$router.push('/Two')">Two</el-button>
    <el-button type="info">Info</el-button>
    <el-button type="warning">Warning</el-button>
    <el-button type="danger">Danger</el-button>
  </div>
</template>
