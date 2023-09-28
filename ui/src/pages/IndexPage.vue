<template>
  <q-page class="">
    <div class="q-pa-md">
      <div>
        <q-btn-toggle
          v-model="modelSelection"
          push
          toggle-color="primary"
          :options="dataList.map((_) => ({ label: _.name, value: _ }))"
        />
      </div>
      <q-card v-if="modelSelection" class="">
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
    </div>

    <ConstraintsVisualization :data="modelSelection?.data" />
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { QTableColumn } from 'quasar';
import { matOpenInNew } from '@quasar/extras/material-icons';
import LeaderLine from 'leader-line-new';
import { IDataModel, dataList } from 'src/services/DefaultModels';
import ConstraintsVisualization from '../components/ConstraintsVisualization.vue';

const modelSelection: Ref<IDataModel | undefined> = ref(undefined);
modelSelection.value = dataList[0];
</script>
