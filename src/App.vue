<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted } from "vue";
import FancyClock from "./components/FancyClock.vue";

onMounted(async () => {
  await listen("refresh", () => {
    location.reload();
  });
});
</script>

<template>
  <div class="desktop flex justify-center items-center h-screen w-screen">
    <FancyClock />
  </div>
</template>

<style></style>
