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
import init, { greet, try_run } from '../transpiler';

async function run() {
  await init();
  greet('hello');
  let code = `
gate add(a, b, c, s) {
    s <| a + b - c;
}

let N = 10;

public input in1;
public input in2;
public output out;

private advice a;
private advice b;
private advice c;

public selector s;

add(a, b, c, s);

a[0] <== in1;
b[0] <== in2;

for i in 0..N {
    if (i > 0) {
        a[i] <== b[i - 1];
        b[i] <== c[i - 1];
    }
    c[i] <== a[i] + b[i];
    s[i] <-- enable;
}

out <== c[N-1];
`;
  const result = try_run({
    k: 4,
    code,
    inputs: { in1: '1', in2: '1' },
  });
  // console.log('result:', result);
}

run();

// import { ref } from 'vue';
// import EssentialLink, { EssentialLinkProps } from 'components/EssentialLink.vue';

// const essentialLinks: EssentialLinkProps[] = [
//   {
//     title: 'Docs',
//     caption: 'quasar.dev',
//     icon: 'school',
//     link: 'https://quasar.dev'
//   },
//   {
//     title: 'Github',
//     caption: 'github.com/quasarframework',
//     icon: 'code',
//     link: 'https://github.com/quasarframework'
//   },
//   {
//     title: 'Discord Chat Channel',
//     caption: 'chat.quasar.dev',
//     icon: 'chat',
//     link: 'https://chat.quasar.dev'
//   },
//   {
//     title: 'Forum',
//     caption: 'forum.quasar.dev',
//     icon: 'record_voice_over',
//     link: 'https://forum.quasar.dev'
//   },
//   {
//     title: 'Twitter',
//     caption: '@quasarframework',
//     icon: 'rss_feed',
//     link: 'https://twitter.quasar.dev'
//   },
//   {
//     title: 'Facebook',
//     caption: '@QuasarFramework',
//     icon: 'public',
//     link: 'https://facebook.quasar.dev'
//   },
//   {
//     title: 'Quasar Awesome',
//     caption: 'Community Quasar projects',
//     icon: 'favorite',
//     link: 'https://awesome.quasar.dev'
//   }
// ];

// const leftDrawerOpen = ref(false)

// function toggleLeftDrawer() {
//   leftDrawerOpen.value = !leftDrawerOpen.value
// }
</script>
