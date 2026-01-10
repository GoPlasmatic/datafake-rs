/**
 * DataFake Playground - Interactive fake data generation
 * Provides inline "Try It" widgets and a full playground page
 */

// WASM module state
let wasmReady = false;
let wasmModule = null;
let initPromise = null;

// Get the base path for loading WASM files
function getBasePath() {
    const scripts = document.getElementsByTagName('script');
    for (let script of scripts) {
        if (script.src && script.src.includes('datafake-playground')) {
            return script.src.replace(/theme\/datafake-playground[^/]*\.js.*$/, '');
        }
    }
    // Fallback: try to detect from current URL by looking for path_to_root
    // mdBook sets this variable in each page
    if (typeof path_to_root !== 'undefined') {
        return window.location.href.replace(/[^/]*$/, '') + path_to_root;
    }
    // Final fallback: try to detect from current URL
    const path = window.location.pathname;
    const parts = path.split('/');
    // Remove last part (current page) and rebuild
    parts.pop();
    return window.location.origin + parts.join('/') + '/';
}

// Initialize WASM module
async function initWasm() {
    if (initPromise) return initPromise;

    initPromise = (async () => {
        try {
            const basePath = getBasePath();
            const wasmJsUrl = basePath + 'wasm/datafake_wasm.js';

            // Dynamic import of WASM JS module
            const module = await import(wasmJsUrl);

            // Initialize WASM - the default export initializes the module
            await module.default();

            wasmModule = module;
            wasmReady = true;
            console.log('DataFake WASM initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize DataFake WASM:', error);
            wasmReady = false;
            return false;
        }
    })();

    return initPromise;
}

// Generate fake data from config
function generateData(config, count = 1) {
    if (!wasmReady || !wasmModule) {
        throw new Error('WASM module not initialized');
    }
    if (count === 1) {
        return wasmModule.generate(config);
    } else {
        const gen = new wasmModule.FakeDataGenerator(config);
        const result = gen.generate_batch(count);
        gen.free();
        return result;
    }
}

// Format JSON for display
function formatJson(str) {
    try {
        const obj = JSON.parse(str);
        return JSON.stringify(obj, null, 2);
    } catch {
        return str;
    }
}

// Validate JSON string
function isValidJson(str) {
    try {
        JSON.parse(str);
        return true;
    } catch {
        return false;
    }
}

// JSON syntax highlighting
function highlightJson(str) {
    // Escape HTML entities
    const escaped = str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');

    // Apply syntax highlighting
    return escaped
        // Strings (including keys in quotes)
        .replace(/"([^"\\]|\\.)*"/g, (match) => {
            // Check if this is a key (followed by :)
            return `<span class="json-string">${match}</span>`;
        })
        // Numbers
        .replace(/\b(-?\d+\.?\d*([eE][+-]?\d+)?)\b/g, '<span class="json-number">$1</span>')
        // Booleans
        .replace(/\b(true|false)\b/g, '<span class="json-boolean">$1</span>')
        // Null
        .replace(/\bnull\b/g, '<span class="json-null">null</span>')
        // Brackets and braces
        .replace(/([{}\[\]])/g, '<span class="json-bracket">$1</span>')
        // Highlight keys (strings followed by :)
        .replace(/<span class="json-string">("([^"\\]|\\.)*")<\/span>(\s*:)/g,
            '<span class="json-key">$1</span>$3');
}

// Sync scroll between textarea and highlight layer
function syncScroll(textarea, highlight) {
    highlight.scrollTop = textarea.scrollTop;
    highlight.scrollLeft = textarea.scrollLeft;
}

// Create an inline "Try It" widget
function createWidget(container) {
    const config = container.dataset.config || '{"schema":{}}';
    const batchSize = parseInt(container.dataset.batch || '1', 10);
    const originalConfig = config;

    const formattedConfig = formatJson(config);

    container.innerHTML = `
        <div class="playground-widget-inner">
            <div class="playground-header">
                <span class="playground-title">Try It</span>
                <div class="playground-actions">
                    <div class="playground-batch-group">
                        <label for="batch-${container.id || Math.random().toString(36).substr(2, 9)}">Count:</label>
                        <input type="number" class="playground-batch" value="${batchSize}" min="1" max="100" />
                    </div>
                    <button class="playground-btn playground-reset" title="Reset to original">Reset</button>
                    <button class="playground-btn playground-run" title="Generate (Ctrl+Enter)">Generate</button>
                </div>
            </div>
            <div class="playground-body">
                <div class="playground-editor-section">
                    <label>Configuration</label>
                    <div class="playground-editor-container">
                        <div class="playground-highlight" aria-hidden="true">${highlightJson(formattedConfig)}</div>
                        <textarea class="playground-config" spellcheck="false">${formattedConfig}</textarea>
                    </div>
                </div>
                <div class="playground-output">
                    <label>Generated Data <span class="generation-info"></span></label>
                    <div class="playground-result"></div>
                </div>
            </div>
        </div>
    `;

    const configInput = container.querySelector('.playground-config');
    const configHighlight = container.querySelector('.playground-highlight');
    const resultDiv = container.querySelector('.playground-result');
    const generationInfo = container.querySelector('.generation-info');
    const runBtn = container.querySelector('.playground-run');
    const resetBtn = container.querySelector('.playground-reset');
    const batchInput = container.querySelector('.playground-batch');

    // Update highlighting on input
    function updateConfigHighlight() {
        configHighlight.innerHTML = highlightJson(configInput.value);
    }

    // Run generation
    function run() {
        const configStr = configInput.value.trim();
        const count = parseInt(batchInput.value, 10) || 1;

        // Validate JSON
        if (!isValidJson(configStr)) {
            resultDiv.className = 'playground-result error';
            resultDiv.textContent = 'Invalid JSON configuration';
            generationInfo.textContent = '';
            return;
        }

        try {
            const result = generateData(configStr, count);
            resultDiv.className = 'playground-result success';
            resultDiv.textContent = formatJson(result);
            generationInfo.textContent = count > 1 ? `(${count} records)` : '(1 record)';
        } catch (error) {
            resultDiv.className = 'playground-result error';
            resultDiv.textContent = 'Error: ' + error.message;
            generationInfo.textContent = '';
        }
    }

    // Reset to original values
    function reset() {
        configInput.value = formatJson(originalConfig);
        batchInput.value = batchSize;
        updateConfigHighlight();
        resultDiv.className = 'playground-result';
        resultDiv.textContent = '';
        generationInfo.textContent = '';
    }

    // Event listeners
    runBtn.addEventListener('click', run);
    resetBtn.addEventListener('click', reset);

    // Input event for highlighting
    configInput.addEventListener('input', updateConfigHighlight);

    // Scroll sync
    configInput.addEventListener('scroll', () => syncScroll(configInput, configHighlight));

    // Keyboard shortcut: Ctrl/Cmd + Enter to run
    // Also prevent arrow keys from triggering mdBook page navigation
    function handleKeydown(e) {
        // Stop propagation for all keys to prevent mdBook navigation
        e.stopPropagation();

        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            run();
        }
    }
    configInput.addEventListener('keydown', handleKeydown);
    batchInput.addEventListener('keydown', handleKeydown);

    // Auto-run on initial load
    if (wasmReady) {
        run();
    }
}

// Initialize all playground widgets on the page
function initPlaygroundWidgets() {
    const widgets = document.querySelectorAll('.playground-widget');
    widgets.forEach(widget => {
        if (!widget.classList.contains('initialized')) {
            createWidget(widget);
            widget.classList.add('initialized');
        }
    });
}

// Create the full playground page
function initFullPlayground() {
    const container = document.getElementById('full-playground');
    if (!container) return;

    // Example templates
    const examples = {
        'Simple User': {
            config: '{"schema":{"id":{"fake":["uuid"]},"name":{"fake":["name"]},"email":{"fake":["email"]}}}'
        },
        'User Profile': {
            config: '{"schema":{"id":{"fake":["uuid"]},"firstName":{"fake":["first_name"]},"lastName":{"fake":["last_name"]},"email":{"fake":["email"]},"age":{"fake":["u8",18,65]},"phone":{"fake":["phone_number"]}}}'
        },
        'Address': {
            config: '{"schema":{"street":{"fake":["street_address"]},"city":{"fake":["city_name"]},"state":{"fake":["state_abbr"]},"zipCode":{"fake":["zip_code"]},"country":{"fake":["country_name"]}}}'
        },
        'Company': {
            config: '{"schema":{"name":{"fake":["company_name"]},"industry":{"fake":["industry"]},"catchPhrase":{"fake":["catch_phrase"]},"website":{"fake":["domain_name"]}}}'
        },
        'E-commerce Product': {
            config: '{"schema":{"id":{"fake":["uuid"]},"name":{"cat":[{"fake":["word"]},"-",{"fake":["word"]}]},"price":{"fake":["f64",9.99,999.99]},"inStock":{"fake":["bool"]},"category":{"fake":["enum","Electronics","Clothing","Books","Home","Sports"]}}}'
        },
        'Financial Transaction': {
            config: '{"schema":{"transactionId":{"fake":["uuid"]},"amount":{"fake":["f64",10.0,10000.0]},"currency":{"fake":["currency_code"]},"iban":{"fake":["iban"]},"bic":{"fake":["bic8"]},"timestamp":{"fake":["datetime"]}}}'
        },
        'Blog Post': {
            config: '{"schema":{"id":{"fake":["uuid"]},"title":{"fake":["sentence",3,6]},"content":{"fake":["paragraph",2,4]},"author":{"fake":["name"]},"publishedAt":{"fake":["datetime"]},"tags":["technology","programming"]}}'
        },
        'With Variables': {
            config: '{"variables":{"userId":{"fake":["uuid"]},"createdAt":{"fake":["datetime"]}},"schema":{"id":{"var":"userId"},"profile":{"userId":{"var":"userId"},"name":{"fake":["name"]}},"metadata":{"createdAt":{"var":"createdAt"},"updatedAt":{"var":"createdAt"}}}}'
        },
        'Network Config': {
            config: '{"schema":{"hostname":{"fake":["domain_name"]},"ipv4":{"fake":["ipv4"]},"ipv6":{"fake":["ipv6"]},"macAddress":{"fake":["mac_address"]},"userAgent":{"fake":["user_agent"]}}}'
        },
        'Credentials': {
            config: '{"schema":{"username":{"fake":["username"]},"password":{"fake":["password",12,20]},"apiKey":{"fake":["alphanumeric",32]}}}'
        }
    };

    const firstExample = Object.keys(examples)[0];
    const formattedConfig = formatJson(examples[firstExample].config);

    container.innerHTML = `
        <div class="full-playground-container">
            <div class="full-playground-header">
                <span class="full-playground-title">DataFake Playground</span>
                <div class="full-playground-controls">
                    <select class="playground-examples">
                        <option value="">Load Example...</option>
                        ${Object.keys(examples).map(name => `<option value="${name}">${name}</option>`).join('')}
                    </select>
                    <div class="playground-batch-group">
                        <label>Count:</label>
                        <input type="number" class="playground-batch" value="1" min="1" max="100" />
                    </div>
                    <button class="playground-btn playground-reset" title="Format JSON">Format</button>
                    <button class="playground-btn playground-reset playground-clear" title="Clear all">Clear</button>
                    <button class="playground-btn playground-run" title="Generate (Ctrl+Enter)">Generate</button>
                </div>
            </div>
            <div class="full-playground-body">
                <div class="full-playground-editor-section">
                    <label>Configuration (JSON)</label>
                    <div class="full-playground-editor-container">
                        <div class="full-playground-highlight" aria-hidden="true">${highlightJson(formattedConfig)}</div>
                        <textarea class="full-playground-config" spellcheck="false" placeholder='{"schema": {"id": {"fake": ["uuid"]}}}'>${formattedConfig}</textarea>
                    </div>
                </div>
                <div class="full-playground-output">
                    <label>Generated Data <span class="generation-info"></span></label>
                    <div class="full-playground-result"></div>
                </div>
            </div>
        </div>
    `;

    const configInput = container.querySelector('.full-playground-config');
    const configHighlight = container.querySelector('.full-playground-highlight');
    const resultDiv = container.querySelector('.full-playground-result');
    const generationInfo = container.querySelector('.generation-info');
    const runBtn = container.querySelector('.playground-run');
    const formatBtn = container.querySelector('.playground-reset:not(.playground-clear)');
    const clearBtn = container.querySelector('.playground-clear');
    const examplesSelect = container.querySelector('.playground-examples');
    const batchInput = container.querySelector('.playground-batch');

    // Update highlighting on input
    function updateConfigHighlight() {
        configHighlight.innerHTML = highlightJson(configInput.value);
    }

    // Run generation
    function run() {
        const configStr = configInput.value.trim();
        const count = parseInt(batchInput.value, 10) || 1;

        if (!configStr) {
            resultDiv.className = 'full-playground-result error';
            resultDiv.textContent = 'Please enter a configuration';
            generationInfo.textContent = '';
            return;
        }

        if (!isValidJson(configStr)) {
            resultDiv.className = 'full-playground-result error';
            resultDiv.textContent = 'Invalid JSON configuration';
            generationInfo.textContent = '';
            return;
        }

        try {
            const result = generateData(configStr, count);
            resultDiv.className = 'full-playground-result success';
            resultDiv.textContent = formatJson(result);
            generationInfo.textContent = count > 1 ? `(${count} records)` : '(1 record)';
        } catch (error) {
            resultDiv.className = 'full-playground-result error';
            resultDiv.textContent = 'Error: ' + error.message;
            generationInfo.textContent = '';
        }
    }

    // Format JSON in editor
    function format() {
        try {
            configInput.value = formatJson(configInput.value);
            updateConfigHighlight();
        } catch {}
    }

    // Clear all
    function clear() {
        configInput.value = '{"schema": {}}';
        batchInput.value = '1';
        updateConfigHighlight();
        resultDiv.className = 'full-playground-result';
        resultDiv.textContent = '';
        generationInfo.textContent = '';
        examplesSelect.value = '';
    }

    // Load example
    function loadExample() {
        const name = examplesSelect.value;
        if (name && examples[name]) {
            configInput.value = formatJson(examples[name].config);
            updateConfigHighlight();
            run();
        }
    }

    // Event listeners
    runBtn.addEventListener('click', run);
    formatBtn.addEventListener('click', format);
    clearBtn.addEventListener('click', clear);
    examplesSelect.addEventListener('change', loadExample);

    // Input event for highlighting
    configInput.addEventListener('input', updateConfigHighlight);

    // Scroll sync
    configInput.addEventListener('scroll', () => syncScroll(configInput, configHighlight));

    // Keyboard shortcut
    // Also prevent arrow keys from triggering mdBook page navigation
    function handleKeydown(e) {
        // Stop propagation for all keys to prevent mdBook navigation
        e.stopPropagation();

        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            run();
        }
    }
    configInput.addEventListener('keydown', handleKeydown);
    batchInput.addEventListener('keydown', handleKeydown);

    // Auto-run on initial load
    if (wasmReady) {
        run();
    }
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', async () => {
    // Show loading state for widgets
    document.querySelectorAll('.playground-widget').forEach(widget => {
        widget.innerHTML = '<div class="playground-loading">Loading playground...</div>';
    });

    const fullPlayground = document.getElementById('full-playground');
    if (fullPlayground) {
        fullPlayground.innerHTML = '<div class="playground-loading">Loading playground...</div>';
    }

    // Initialize WASM
    const success = await initWasm();

    if (success) {
        // Initialize widgets
        initPlaygroundWidgets();
        initFullPlayground();
    } else {
        // Show error state
        document.querySelectorAll('.playground-widget').forEach(widget => {
            widget.innerHTML = '<div class="playground-error">Failed to load playground. Please refresh the page.</div>';
        });
        if (fullPlayground) {
            fullPlayground.innerHTML = '<div class="playground-error">Failed to load playground. Please refresh the page.</div>';
        }
    }
});

// Re-initialize widgets when page content changes (for mdBook's navigation)
if (typeof window !== 'undefined') {
    // MutationObserver to detect page changes
    const observer = new MutationObserver((mutations) => {
        if (wasmReady) {
            initPlaygroundWidgets();
            initFullPlayground();
        }
    });

    // Start observing when DOM is ready
    document.addEventListener('DOMContentLoaded', () => {
        const content = document.getElementById('content');
        if (content) {
            observer.observe(content, { childList: true, subtree: true });
        }
    });
}
