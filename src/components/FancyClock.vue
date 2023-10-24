<script setup lang="ts" >
import { ref } from "vue";

const day = ref("MONDAY");
const hr = ref(0);
const min = ref(0);
const sec = ref(0);

const days = [
  "SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"
]

const refresh = () => {
  let date = new Date();
  hr.value = date.getHours();
  min.value = date.getMinutes();
  sec.value = date.getSeconds();
  day.value = days[date.getDay()];
}

refresh();

setInterval(refresh, 1000);

</script>

<template>
  <div class="flex flex-col gap-2 text-center select-none text-blue-950 text-opacity-80 pb-24">
    <div class="day pl-8 text-8xl">{{ day }}</div>
    <div class="date text-3xl">{{ String(hr).padStart(2, '0') }}:{{ String(min).padStart(2, '0') }}:{{ String(sec).padStart(2, '0') }}</div>
  </div>
</template>

<style>
.day {
  font-family: "Anurati Regular";
  letter-spacing: 2rem;
}
.date {
  font-family: "Space Mono";
  letter-spacing: 0.25rem;
}
</style>