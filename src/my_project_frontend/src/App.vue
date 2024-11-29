<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let displayChat = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const msg = target.querySelector('#msg').value;
  //displayChat.value = msg;
  await my_project_backend.save_msg(msg);
  await getMsgs();
}

async function getMsgs() {
  const chat = await my_project_backend.get_chat();
  console.log(chat);
  displayChat.value = chat;
}

getMsgs();
</script>

<template>
  <main class="flex flex-col items-center justify-center h-dvh">
    <content class="w-1/3">
      <img src="/logo2.svg" alt="DFINITY logo" class="w-full" />
      <br />
      <br />
      <form action="#" @submit="handleSubmit">
        <label for="msg">Enter your msg: &nbsp;</label>
        <input id="msg" alt="msg" type="text" class="border-black border rounded-md m-5 p-1" />
        <button type="submit" class="border p-1 rounded-md">Click Me!</button>
      </form>
      <section id="displayChat" class="border w-full rounded-md">
        <div v-for="msg in displayChat" class="p-5 border-b last:border-b-0">{{ msg }}</div>
      </section>
    </content>
  </main>
</template>
