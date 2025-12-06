<template>
  <div class="calculator-view page-container">
    <header class="app-header">
      <button class="back-btn" @click="router.push('/')">
        <ArrowLeft />
      </button>
      <h1>{{ t('calculator_app') }}</h1>
    </header>

    <div class="calc-container">
      <!-- Sidebar / Tabs -->
      <nav class="calc-sidebar">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          :class="{ active: currentTab === tab.id }"
          @click="currentTab = tab.id"
        >
          <component :is="tab.icon" :size="20" />
          <span>{{ t(tab.label_key) }}</span>
        </button>
      </nav>

      <!-- Content Area -->
      <main class="calc-content">
        
        <!-- 1. Scientific Calculator -->
        <div v-if="currentTab === 'science'" class="tool-panel calculator-panel">
          <div class="display">{{ calcDisplay }}</div>
          <div class="keypad">
            <button @click="clearCalc" class="op-btn">C</button>
            <button @click="appendCalc('(')" class="op-btn">(</button>
            <button @click="appendCalc(')')" class="op-btn">)</button>
            <button @click="appendCalc('/')" class="op-btn">÷</button>
            
            <button @click="appendCalc('7')">7</button>
            <button @click="appendCalc('8')">8</button>
            <button @click="appendCalc('9')">9</button>
            <button @click="appendCalc('*')" class="op-btn">×</button>
            
            <button @click="appendCalc('4')">4</button>
            <button @click="appendCalc('5')">5</button>
            <button @click="appendCalc('6')">6</button>
            <button @click="appendCalc('-')" class="op-btn">-</button>
            
            <button @click="appendCalc('1')">1</button>
            <button @click="appendCalc('2')">2</button>
            <button @click="appendCalc('3')">3</button>
            <button @click="appendCalc('+')" class="op-btn">+</button>
            
            <button @click="appendCalc('0')" class="wide">0</button>
            <button @click="appendCalc('.')">.</button>
            <button @click="calculate" class="action-btn">=</button>
          </div>
          <div class="sci-keys">
            <button @click="appendFunc('sin(')">sin</button>
            <button @click="appendFunc('cos(')">cos</button>
            <button @click="appendFunc('tan(')">tan</button>
            <button @click="appendFunc('sqrt(')">√</button>
            <button @click="appendFunc('log(')">log</button>
            <button @click="appendFunc('pow(')">^</button>
            <button @click="appendConst('PI')">π</button>
            <button @click="appendConst('E')">e</button>
          </div>
        </div>

        <!-- 2. Base64 Converter -->
        <div v-else-if="currentTab === 'base64'" class="tool-panel">
          <div class="input-group">
            <label>{{ t('text_input') }}</label>
            <textarea v-model="base64Input" rows="4" :placeholder="t('enter_text')"></textarea>
          </div>
          <div class="actions">
            <button @click="toBase64">Encode &darr;</button>
            <button @click="fromBase64">Decode &uarr;</button>
          </div>
          <div class="input-group">
            <label>Base64</label>
            <textarea v-model="base64Output" rows="4" placeholder="Result..."></textarea>
          </div>
        </div>

        <!-- 3. Hash Calculator -->
        <div v-else-if="currentTab === 'hash'" class="tool-panel">
          <div class="input-group">
            <label>{{ t('text_input') }}</label>
            <textarea v-model="hashInput" rows="3"></textarea>
          </div>
          <div class="actions">
             <select v-model="hashAlgo">
               <option value="SHA-1">SHA-1</option>
               <option value="SHA-256">SHA-256</option>
               <option value="SHA-384">SHA-384</option>
               <option value="SHA-512">SHA-512</option>
             </select>
             <button @click="computeHash">{{ t('calculate') }}</button>
          </div>
          <div class="input-group">
            <label>Hash (Hex)</label>
            <input readonly v-model="hashOutputHex" class="code-input" />
          </div>
          <div class="input-group">
            <label>Hash (Base64)</label>
            <input readonly v-model="hashOutputBase64" class="code-input" />
          </div>
        </div>

        <!-- 4. Timestamp Converter -->
        <div v-else-if="currentTab === 'timestamp'" class="tool-panel">
           <div class="section">
             <h3>{{ t('now') }}</h3>
             <div class="display-box">{{ nowTimestamp }}</div>
             <button @click="updateNow">{{ t('refresh') }}</button>
           </div>
           
           <div class="section">
             <h3>{{ t('timestamp_to_date') }}</h3>
             <div class="row">
               <input v-model="tsInput" type="number" placeholder="Timestamp (ms or s)" />
               <select v-model="tsUnit">
                 <option value="ms">ms</option>
                 <option value="s">s</option>
               </select>
             </div>
             <div class="result">{{ tsDateOutput }}</div>
           </div>

           <div class="section">
             <h3>{{ t('date_to_timestamp') }}</h3>
             <input type="datetime-local" v-model="dateInput" />
             <div class="result">{{ dateToTsOutput }}</div>
           </div>
        </div>

        <!-- 5. Base Converter -->
        <div v-else-if="currentTab === 'base'" class="tool-panel">
          <div class="input-group">
            <label>Decimal (10)</label>
            <input v-model="baseDec" type="number" @input="convertBase('dec')" />
          </div>
          <div class="input-group">
            <label>Hexadecimal (16)</label>
            <input v-model="baseHex" @input="convertBase('hex')" />
          </div>
          <div class="input-group">
            <label>Binary (2)</label>
            <input v-model="baseBin" @input="convertBase('bin')" />
          </div>
          <div class="input-group">
            <label>Octal (8)</label>
            <input v-model="baseOct" @input="convertBase('oct')" />
          </div>
        </div>

      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { ArrowLeft, Calculator, Binary, FileLock2, Clock, ArrowRightLeft } from 'lucide-vue-next';
import { useI18n } from '../composables/useI18n';

const router = useRouter();
const { t } = useI18n();

const currentTab = ref('science');
const tabs = [
  { id: 'science', label_key: 'calc_science', icon: Calculator },
  { id: 'base64', label_key: 'calc_base64', icon: Binary },
  { id: 'hash', label_key: 'calc_hash', icon: FileLock2 },
  { id: 'timestamp', label_key: 'calc_time', icon: Clock },
  { id: 'base', label_key: 'calc_base', icon: ArrowRightLeft },
];

// --- Scientific Calculator ---
const calcDisplay = ref('');
const appendCalc = (char: string) => {
  calcDisplay.value += char;
};
const clearCalc = () => {
  calcDisplay.value = '';
};
const appendFunc = (func: string) => {
  calcDisplay.value += func;
};
const appendConst = (c: string) => {
  if (c === 'PI') calcDisplay.value += Math.PI;
  if (c === 'E') calcDisplay.value += Math.E;
};
const calculate = () => {
  try {
    // Safety check: simplified eval equivalent
    // Replace math functions with Math.func
    let expr = calcDisplay.value
      .replace(/sin\(/g, 'Math.sin(')
      .replace(/cos\(/g, 'Math.cos(')
      .replace(/tan\(/g, 'Math.tan(')
      .replace(/sqrt\(/g, 'Math.sqrt(')
      .replace(/log\(/g, 'Math.log10(')
      .replace(/pow\(/g, 'Math.pow(');
    
    // eslint-disable-next-line no-new-func
    const result = new Function('return ' + expr)();
    calcDisplay.value = String(result);
  } catch (e) {
    calcDisplay.value = 'Error';
  }
};

// --- Base64 ---
const base64Input = ref('');
const base64Output = ref('');
const toBase64 = () => {
  try {
    base64Output.value = btoa(unescape(encodeURIComponent(base64Input.value)));
  } catch(e) {
    base64Output.value = 'Error';
  }
};
const fromBase64 = () => {
  try {
    base64Input.value = decodeURIComponent(escape(atob(base64Output.value)));
  } catch(e) {
    base64Input.value = 'Error';
  }
};

// --- Hash ---
const hashInput = ref('');
const hashOutputHex = ref(''); // Renamed from hashOutput
const hashOutputBase64 = ref('');
const hashAlgo = ref('SHA-256');

// Helper to convert ArrayBuffer to Base64 string
const arrayBufferToBase64 = (buffer: ArrayBuffer) => {
  let binary = '';
  const bytes = new Uint8Array(buffer);
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i] as number);
  }
  return btoa(binary);
};

const computeHash = async () => {
  const encoder = new TextEncoder();
  const data = encoder.encode(hashInput.value);
  const hash = await crypto.subtle.digest(hashAlgo.value, data);
  const hashArray = Array.from(new Uint8Array(hash));

  // Hex output
  hashOutputHex.value = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

  // Base64 output
  hashOutputBase64.value = arrayBufferToBase64(hash);
};

// --- Timestamp ---
const nowTimestamp = ref(Date.now());
const timer = setInterval(() => { nowTimestamp.value = Date.now(); }, 1000);
onUnmounted(() => clearInterval(timer));
const updateNow = () => nowTimestamp.value = Date.now();

const tsInput = ref('');
const tsUnit = ref('ms');
const tsDateOutput = computed(() => {
  const val = parseInt(tsInput.value);
  if (isNaN(val)) return '-';
  const ms = tsUnit.value === 's' ? val * 1000 : val;
  return new Date(ms).toLocaleString();
});

const dateInput = ref('');
const dateToTsOutput = computed(() => {
  if (!dateInput.value) return '-';
  return new Date(dateInput.value).getTime();
});

// --- Base Converter ---
const baseDec = ref('');
const baseHex = ref('');
const baseBin = ref('');
const baseOct = ref('');

const convertBase = (source: 'dec' | 'hex' | 'bin' | 'oct') => {
  let val = 0;
  if (source === 'dec') val = parseInt(baseDec.value, 10);
  if (source === 'hex') val = parseInt(baseHex.value, 16);
  if (source === 'bin') val = parseInt(baseBin.value, 2);
  if (source === 'oct') val = parseInt(baseOct.value, 8);

  if (isNaN(val)) {
    baseDec.value = baseHex.value = baseBin.value = baseOct.value = '';
    return;
  }

  if (source !== 'dec') baseDec.value = val.toString(10);
  if (source !== 'hex') baseHex.value = val.toString(16).toUpperCase();
  if (source !== 'bin') baseBin.value = val.toString(2);
  if (source !== 'oct') baseOct.value = val.toString(8);
};

</script>

<style scoped>
.calculator-view {
  background: var(--window-bg);
  height: 100%;
  display: flex;
  flex-direction: column;
  color: var(--text-color);
}

.app-header {
  display: flex;
  align-items: center;
  padding: 35px 20px 15px 20px;
  gap: 15px;
  background: var(--header-bg);
}

.back-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
}

h1 {
  font-size: 1.2rem;
  margin: 0;
}

.calc-container {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.calc-sidebar {
  width: 80px;
  background: var(--sidebar-bg);
  display: flex;
  flex-direction: column;
  padding: 10px;
  gap: 10px;
  border-right: 1px solid var(--border-color-soft);
}

.calc-sidebar button {
  background: transparent;
  border: none;
  color: var(--sidebar-button-text-inactive);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
  padding: 10px 5px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.7rem;
  text-align: center;
}

.calc-sidebar button.active {
  background: var(--sidebar-button-bg-active);
  color: var(--accent-color, #007aff);
}

.calc-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.tool-panel {
  max-width: 600px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* Calculator Specific */
.calculator-panel {
  max-width: 320px;
}

.display {
  background: var(--calc-display-bg);
  padding: 20px;
  font-size: 2rem;
  text-align: right;
  border-radius: 12px;
  min-height: 80px;
  word-break: break-all;
}

.keypad {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}

.keypad button, .sci-keys button {
  background: var(--calc-button-bg);
  border: none;
  color: var(--text-color);
  padding: 15px 0;
  border-radius: 50%;
  font-size: 1.2rem;
  cursor: pointer;
  transition: background 0.1s;
}

.keypad button:active {
  background: var(--calc-button-active-bg);
}

.keypad button.op-btn {
  background: var(--calc-operator-button-bg);
  color: white;
}

.keypad button.action-btn {
  background: var(--calc-action-button-bg);
}

.keypad button.wide {
  grid-column: span 2;
  border-radius: 30px;
}

.sci-keys {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
  margin-top: 10px;
}

.sci-keys button {
  font-size: 0.9rem;
  padding: 10px 0;
  border-radius: 8px;
  background: rgba(255,255,255,0.05);
}

/* Common Inputs */
.input-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-group label {
  font-size: 0.9rem;
  opacity: 0.7;
}

textarea, input, select {
  background: var(--input-bg);
  border: 1px solid var(--input-border);
  color: var(--text-color);
  padding: 10px;
  border-radius: 8px;
  font-family: monospace;
}

.actions {
  display: flex;
  gap: 10px;
}

button {
  padding: 10px 20px;
  border-radius: 8px;
  border: none;
  background: var(--accent-color, #007aff);
  color: var(--button-text-color);
  cursor: pointer;
}

.code-input {
  font-family: monospace;
}

.row {
  display: flex;
  gap: 10px;
}

.result {
  background: var(--result-bg);
  padding: 10px;
  border-radius: 8px;
  min-height: 40px;
  display: flex;
  align-items: center;
}

.section h3 {
  font-size: 1rem;
  margin-bottom: 10px;
  opacity: 0.8;
  border-bottom: 1px solid var(--border-color-soft);
  padding-bottom: 5px;
}
</style>
