<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted } from "vue";
import FancyClock from "./components/FancyClock.vue";
import { WebviewWindow } from "@tauri-apps/api/window";

const openSettings = () => {
  console.log("settings");
  const settingsWindow = new WebviewWindow("settings", {
    url: "https://www.google.com",
    title: "Settings",
    width: 800,
    height: 600,
    alwaysOnTop: true,
  });
  
  settingsWindow.once("tauri://created", () => {
    console.log("settings window created");
  });
};

onMounted(async () => {
  await listen("trayEvent", (event) => {
    if (event.payload == "refresh") {
      location.reload();
    } else if (event.payload == "settings") {
      openSettings();
    }
  });
});
</script>

<template>
  <div class="desktop flex justify-center items-center h-screen w-screen">
    <FancyClock />
  </div>
</template>

<style></style>
