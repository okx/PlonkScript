<template>
  <q-page class="">
    <div class="q-pa-md">
      <div>
        <q-btn-toggle
          v-model="modelSelection"
          push
          toggle-color="primary"
          :options="([{ label: 'Custom', value: undefined }] as any).concat(dataList.map((_) => ({ label: _.name, value: _ })))"
        />
      </div>
      <q-card v-if="modelSelection && modelSelection.name != 'Custom'" class="">
        <q-card-section>
          <div v-if="modelSelection?.title" class="text-h6">
            {{ modelSelection.title }}
          </div>
        </q-card-section>

        <q-card-section v-if="modelSelection?.description"
          >{{ modelSelection.description }}
        </q-card-section>

        <q-separator v-if="modelSelection?.sourceUrl" />

        <q-card-actions>
          <q-btn
            flat
            v-if="modelSelection?.sourceUrl"
            :href="modelSelection.sourceUrl"
            target="_blank"
            :icon="matOpenInNew"
            >See Source Code</q-btn
          >
        </q-card-actions>
      </q-card>

      <q-card v-else class="">
        <q-card-section>
          <div class="text-h6">Custom</div>
        </q-card-section>

        <q-card-section>
          Select custom model which produced by the MockProver, inject your code
          like:
          <pre>
let d = format!("{:#?}", prover);
let mut file = std::fs::File::create("data.rust").unwrap();
std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();</pre
          >

          PS: this is client-only processing, no data is transfered to the
          server.
        </q-card-section>
        <q-separator />
        <q-card-actions>
          <q-uploader ref="uploaderRef" :multiple="false" @added="onFileAdded">
            <template v-slot:header="scope">
              <div class="row no-wrap items-center q-pa-sm q-gutter-xs">
                <div class="col">
                  <div class="q-uploader__title">Select the debug output</div>
                </div>
                <q-btn
                  v-if="scope.canAddFiles"
                  type="a"
                  icon="add_box"
                  @click="scope.pickFiles"
                  round
                  dense
                  flat
                >
                  <q-uploader-add-trigger />
                  <q-tooltip>Pick Files</q-tooltip>
                </q-btn>
              </div>
            </template>
          </q-uploader>
        </q-card-actions>
      </q-card>
    </div>

    <ConstraintsVisualization :data="modelSelection?.data" />
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import { matOpenInNew } from '@quasar/extras/material-icons';
import { IDataModel, dataList } from 'src/services/DefaultModels';
import ConstraintsVisualization from '../components/ConstraintsVisualization.vue';
import { QUploader } from 'quasar';
import { useQuasar } from 'quasar';
import { convertMockProverOutputToObject } from 'src/services/MockProverTranslator';

const $q = useQuasar();

const modelSelection: Ref<IDataModel | undefined> = ref(undefined);
const uploaderRef: Ref<QUploader | null> = ref(null);

function onFileAdded(files: readonly File[]) {
  var reader = new FileReader();
  reader.onload = function (event) {
    uploaderRef.value?.reset();
    const result = event.target?.result;
    if (
      !result ||
      typeof result != 'string' ||
      !result.startsWith('MockProver')
    ) {
      $q.notify({
        message: 'Invalid file, only MockProver debug output is supported.',
        type: 'negative',
      });
      return;
    }

    const data = convertMockProverOutputToObject(result);
    modelSelection.value = { name: 'Custom', data };
  };
  reader.readAsText(files[0]);
}
</script>
