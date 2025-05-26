<script setup>
import { ref, onMounted } from 'vue'
import init, { get_cell_data, get_visible_cells } from '../../wasm-canvas/pkg/wasm_canvas.js';

const loading = ref(true)
const canvasRef = ref(null)

onMounted(async () => {
  init().then(() => {
    loading.value = false;
    const canvas = canvasRef.value;
    const ctx = canvas.getContext('2d');

    const totalRows = 1000000;
    const totalCols = 30;
    const rowHeight = 20;
    const colWidth = 100;

    function draw(scrollTop) {
      const visibleRows = get_visible_cells(scrollTop, rowHeight, canvas.height, totalRows);
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      visibleRows.forEach((row, i) => {
        for (let col = 0; col < totalCols; col++) {
          const text = get_cell_data(row, col);
          ctx.strokeRect(col * colWidth, i * rowHeight, colWidth, rowHeight);
          ctx.fillText(text, col * colWidth + 5, i * rowHeight + 15);
        }
      });
    }

    let scrollTop = 0;
    draw(scrollTop);

    canvas.addEventListener('wheel', (e) => {
      scrollTop += e.deltaY;
      if (scrollTop < 0) scrollTop = 0;
      draw(scrollTop);
    });
  });

})
</script>

<template>
  <div v-if="loading">Loading WASM...</div>
  <canvas ref="canvasRef" width="1000" height="600"></canvas>
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
