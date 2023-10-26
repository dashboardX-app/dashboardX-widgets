<script setup lang="ts" >
import { ref } from "vue";

const day = ref("MONDAY");
const hr = ref(0);
const min = ref(0);
const sec = ref(0);
const datetext = ref("");

const days = [
  "SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"
]

const refresh = () => {
  let date = new Date();
  hr.value = date.getHours();
  min.value = date.getMinutes();
  sec.value = date.getSeconds();
  day.value = days[date.getDay()];
  datetext.value = String(date.getDate()).padStart(2, '0') + "/" + String(date.getMonth() + 1).padStart(2, '0') + "/" + String(date.getFullYear()).padStart(4, '0');
}

refresh();

setInterval(refresh, 1000);

</script>

<template>
  <div class="flex flex-col gap-2 text-center select-none text-slate-900 text-opacity-80 pb-4">
    <div class="day pl-8 text-8xl">{{ day }}</div>
    <div class="datetime text-2xl">{{ datetext }} ⚬ {{ String(hr).padStart(2, '0') }}:{{ String(min).padStart(2, '0') }}:{{ String(sec).padStart(2, '0') }}</div>
  </div>
</template>

<style>
.day {
  font-family: "Anurati Regular";
  letter-spacing: 2rem;
}
.datetime {
  font-family: "Space Mono";
  letter-spacing: 0.25rem;
}
</style>