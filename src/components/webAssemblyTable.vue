<script setup>
import { ref, onMounted } from 'vue'
import init, { process_data } from '../../wasm-processor/pkg/wasm_processor'

const data = ref([])
const loading = ref(true)

onMounted(async () => {
  await init()
  const jsonStr = process_data("100000") // 生成1000条测试数据
  data.value = JSON.parse(jsonStr)
  loading.value = false
})
</script>

<template>
  <div v-if="loading">Loading WASM...</div>
  <table v-else class="wasm-table">
    <thead>
      <tr>
        <th>ID</th>
        <th>Processed Value</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="item in data" :key="item.id">
        <td>{{ item.id }}</td>
        <td>{{ item.processed }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style>
.wasm-table {
  width: 100%;
  border-collapse: collapse;
}
.wasm-table th, .wasm-table td {
  border: 1px solid #ddd;
  padding: 8px;
}
</style>
