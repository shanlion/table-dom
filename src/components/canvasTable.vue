<template>
  <canvas ref="canvasRef" width="800" height="500"></canvas>
</template>

<script setup>
import { onMounted, ref } from 'vue'
const tableCols = Array.from({length: 30}, (_, i) => i + 1)
const tableData = Array.from({length: 1000}, (_, i) => i + 1)

const canvasRef = ref(null)

onMounted(() => {
  const canvas = canvasRef.value

  const dpr = window.devicePixelRatio || 1
  
  const ctx = canvas.getContext('2d')
  ctx.scale(dpr, dpr)
  
  drawCells(ctx)
})

const drawCells = (ctx) => {
  ctx.beginPath()
  for (let rowIndex = 0; rowIndex < tableData.length; rowIndex++) {
    for (let index = 0; index < tableCols.length; index++) {
      ctx.moveTo(index * 100, rowIndex * 30)
      ctx.lineTo((index + 1) * 100, rowIndex * 30)
      ctx.lineTo((index + 1) * 100, (rowIndex + 1) * 30)
      ctx.lineTo(index * 100, (rowIndex + 1) * 30)
      ctx.lineTo(index * 100, rowIndex * 30)
      ctx.fillText(`row:${rowIndex} col${index}`, index * 100 + 5, rowIndex * 30 + 20)
    }
  }
  ctx.stroke()
  ctx.closePath()
}
</script>
