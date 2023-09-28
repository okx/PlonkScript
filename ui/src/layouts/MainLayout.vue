<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <!-- <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        /> -->

        <q-toolbar-title> Halo2 Visualizer </q-toolbar-title>

        <div>Demo Version</div>
      </q-toolbar>
    </q-header>

    <!-- <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          Essential Links
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer> -->

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
// const rust = import('../transpiler/wasm_binding');

// rust.then((m) => m.greet("hello")).catch(console.error);
// import init, { greet, try_run } from '../transpiler';

// async function run() {
//   await init();
//   greet('hello');
//   // console.log('result:', result);
// }

// run();
</script>
