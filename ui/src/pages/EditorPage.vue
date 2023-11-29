<template>
  <q-page class="">
    <q-splitter
      v-model="splitPercent"
      style="height: 90vh"
      @update:model-value="updatePanel()"
      :limits="[0, Infinity]"
    >
      <template v-slot:before>
        <div class="q-pa-md">
          <div id="editorRef" ref="editorRef" style="height: 80vh"></div>
        </div>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <ConstraintsVisualization :data="vis" />
        </div>
      </template>
    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import { language, theme } from 'src/services/PlonkScriptLanguage';
import init, { try_run } from '../transpiler';
import { convertMockProverOutputToObject } from 'src/services/MockProverTranslator';
import { MockProverData } from 'src/services/ConstraintSystem';
import ConstraintsVisualization from '../components/ConstraintsVisualization.vue';
self.MonacoEnvironment = {
  getWorker() {
    return new editorWorker();
  },
};

monaco.languages.register({ id: 'plonkscript' });
monaco.languages.setMonarchTokensProvider('plonkscript', language);
    monaco.editor.defineTheme('plonkscript', theme);
    // monaco.editor.setModelMarkers();

const splitPercent = ref(50);
const editorRef = ref(null);

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

// Press Ctrl+Enter to execute the code
`;
const vis: Ref<MockProverData | undefined> = ref(undefined);

let editor: monaco.editor.IStandaloneCodeEditor | undefined = undefined;

setTimeout(() => {
  if (editorRef.value) {
    // console.log('combine');
    editor = monaco.editor.create(editorRef.value as HTMLElement, {
      value: code,
      language: 'plonkscript',
      theme: 'plonkscript',
      minimap: {
        enabled: false,
      },
    });

    editor.addAction({
      id: 'runCode',

      label: 'RunCode',

      keybindings: [
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyR,
        monaco.KeyMod.WinCtrl | monaco.KeyCode.Enter,
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
      ],

      contextMenuGroupId: 'debug',

      contextMenuOrder: 1.5,

      run: function (ed) {
        const code = ed.getValue();
        const result = try_run({
          k: 4,
          code,
          inputs: { in1: '1', in2: '1' },
        });
        vis.value = convertMockProverOutputToObject(result);
      },
    });
  }
}, 300);

init();

function updatePanel() {
  if (!editor) return;
  editor.layout();
}
</script>
