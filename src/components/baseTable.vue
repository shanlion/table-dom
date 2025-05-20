<template>
  <table>
    <thead>
      <tr>
        <th v-for="(_, i) in tableCols" :key="i">Column {{ i + 1 }}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(_, i) in tableData" :key="i">
        <td v-for="(_, j) in tableCols" :key="j">
          Row {{ i + 1 }}, Column {{ j + 1 }}
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script setup>
const tableCols = Array.from({length: 30}, (_, i) => i + 1)
const tableData = Array.from({length: 1000}, (_, i) => i + 1)

</script>
