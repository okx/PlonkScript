<template>
  <q-page class="">
    <q-splitter v-model="splitPercent" style="height: 90vh">
      <template v-slot:before>
        <div class="q-pa-md">
          <div id="editor" ref="editor" style="height: 80vh"></div>
        </div>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <div class="text-h4 q-mb-md">After</div>
          <div v-for="n in 20" :key="n" class="q-my-md">
            {{ n }}. Lorem ipsum dolor sit, amet consectetur adipisicing elit.
            Quis praesentium cumque magnam odio iure quidem, quod illum numquam
            possimus obcaecati commodi minima assumenda consectetur culpa fuga
            nulla ullam. In, libero.
          </div>
        </div>
      </template>
    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { QTableColumn } from 'quasar';
import { matOpenInNew } from '@quasar/extras/material-icons';
import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
import { language } from 'src/services/PlonkScriptLanguage';
self.MonacoEnvironment = {
  getWorker(_, label) {
    // if (label === 'json') {
    //   return new jsonWorker()
    // }
    // if (label === 'css' || label === 'scss' || label === 'less') {
    //   return new cssWorker()
    // }
    // if (label === 'html' || label === 'handlebars' || label === 'razor') {
    //   return new htmlWorker()
    // }
    // if (label === 'typescript' || label === 'javascript') {
    //   return new tsWorker()
    // }
    return new editorWorker();
  },
};

// Register a new language
monaco.languages.register({ id: 'plonkscript' });

// Register a tokens provider for the language
monaco.languages.setMonarchTokensProvider('plonkscript', language);

const splitPercent = ref(50);
const editor = ref(null);

const code = `
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
setTimeout(() => {
  if (editor.value) {
    console.log('combine');
    monaco.editor.create(editor.value as HTMLElement, {
      value: code,
      language: 'plonkscript',
      // theme: 'vs-dark',
      minimap: {
        enabled: false,
      },
    });
  }
}, 300);
</script>

<style scoped lang="scss">
//
</style>
