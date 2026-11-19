

        function dispatchAction(actionName, targetEl, event) {
            if (!actionName) return;
            switch (actionName) {
                case 'switchTab':
                    if (targetEl) {
                        const t = targetEl.getAttribute('data-tab') || 'ask';
                        switchTab(t);
                    }
                    break;
                case 'smartRun':
                    smartRun();
                    break;
                case 'checkFile':
                    checkFile();
                    break;
                case 'runTests':
                    runTests();
                    break;
                case 'explainCode':
                    explainCode();
                    break;
                case 'auditFile':
                    auditFile();
                    break;
                case 'generateTests':
                    generateTests();
                    break;
                case 'openArch':
                    switchTab('arch');
                    break;
                case 'analyzeWorkspace':
                    analyzeWorkspace();
                    break;
                case 'requestWorkspaceRefactor':
                    requestWorkspaceRefactor();
                    break;
                case 'toggleConfigPanel':
                    toggleConfigPanel();
                    break;
                case 'saveAllSettings':
                    saveAllSettings();
                    break;
                case 'testConnection':
                    testConnection();
                    break;
                case 'triggerFilePicker':
                    triggerFilePicker();
                    break;
                case 'stopGeneration':
                    stopGeneration();
                    break;
                case 'sendPrompt':
                    sendPrompt();
                    break;
                case 'refreshOllamaModels':
                    refreshOllamaModels();
                    break;
                case 'requestGitHubLogin':
                    requestGitHubLogin();
                    break;
                case 'saveGitHubConfig':
                    saveGitHubConfig();
                    break;
                case 'saveOllamaConfig':
                    saveOllamaConfig();
                    break;
                case 'togglePassword':
                    if (targetEl) {
                        const targetId = targetEl.getAttribute('data-target');
                        if (targetId) togglePasswordVisibility(targetId);
                    }
                    break;
                case 'insertSnippet':
                    if (targetEl) {
                        const snipId = targetEl.getAttribute('data-snippet');
                        if (snipId) insertSnippet(snipId);
                    }
                    break;
                case 'cloneExample':
                    if (targetEl) {
                        const slug = targetEl.getAttribute('data-slug') || targetEl.closest('.example-card')?.getAttribute('data-slug');
                        if (slug) cloneExample(slug);
                    }
                    break;
                case 'openFile':
                    if (targetEl) {
                        const fp = targetEl.getAttribute('data-filepath');
                        if (fp) openFile(fp);
                    }
                    break;
                case 'previewDiff':
                    if (targetEl) {
                        const fp = targetEl.getAttribute('data-filepath');
                        const pid = targetEl.getAttribute('data-proposal-id');
                        previewDiff(fp, pid);
                    }
                    break;
                case 'toggleInlineDiff':
                    if (targetEl) {
                        const targetId = targetEl.getAttribute('data-target');
                        const el = document.getElementById(targetId);
                        if (el) {
                            const isHidden = el.style.display === 'none' || !el.style.display;
                            el.style.display = isHidden ? 'block' : 'none';
                            targetEl.innerText = isHidden ? 'Hide' : 'View';
                        }
                    }
                    break;
                case 'setFileStatus':
                    if (targetEl) {
                        const pid = targetEl.getAttribute('data-proposal-id');
                        const fp = targetEl.getAttribute('data-filepath');
                        const status = targetEl.getAttribute('data-status');
                        if (pid && fp && status) setFileStatus(pid, fp, status);
                    }
                    break;
                case 'acceptProposal':
                    if (targetEl) {
                        const pid = targetEl.getAttribute('data-proposal-id');
                        if (pid) acceptProposal(pid);
                    }
                    break;
                case 'rejectProposal':
                    if (targetEl) {
                        const pid = targetEl.getAttribute('data-proposal-id');
                        if (pid) rejectProposal(pid);
                    }
                    break;
                case 'copyText':
                    if (targetEl) {
                        const txt = targetEl.getAttribute('data-text');
                        if (txt) copyText(txt);
                    }
                    break;
                case 'attachActiveFile':
                    vscode.postMessage({ type: 'attachActiveFile' });
                    break;
                case 'newChat':
                    startNewChat();
                    break;
                case 'executeRefactorPlan':
                    if (targetEl) {
                        const goal = targetEl.getAttribute('data-goal');
                        const targetPath = targetEl.getAttribute('data-target-path');
                        const planId = targetEl.getAttribute('data-plan-id');
                        showAgentTracker('ExecutingPlan', 'Generating coordinated code refactoring and diffs...');
                        vscode.postMessage({
                            type: 'executeRefactorPlan',
                            planId: planId,
                            goal: goal,
                            targetPath: targetPath
                        });
                        const planCard = document.getElementById('plan-' + planId);
                        if (planCard) {
                            const actBox = planCard.querySelector('.proposal-global-actions');
                            if (actBox) actBox.innerHTML = '<div style="color: #60a5fa; font-weight: 600; text-align: center; grid-column: span 2;">Plan Approved - Generating Diffs...</div>';
                        }
                    }
                    break;
                case 'cancelRefactorPlan':
                    if (targetEl) {
                        const planId = targetEl.getAttribute('data-plan-id');
                        const planCard = document.getElementById('plan-' + planId);
                        if (planCard) {
                            planCard.style.borderColor = '#ef4444';
                            const actBox = planCard.querySelector('.proposal-global-actions');
                            if (actBox) actBox.innerHTML = '<div style="color: #ef4444; font-weight: 600; text-align: center; grid-column: span 2;">Plan Cancelled by User. No files were modified.</div>';
                        }
                    }
                    break;
                default:
                    if (typeof window[actionName] === 'function') {
                        window[actionName]();
                    }
                    break;
            }
        }


        let vscode;
        try {
            vscode = acquireVsCodeApi();
        } catch (e) {
            vscode = {
                postMessage: () => {},
                getState: () => ({ history: [], activeTab: 'ask' }),
                setState: () => {}
            };
        }

        const TAB_CONFIG = {
            ask: {
                buttonId: 'tab-nav-ask',
                sectionId: 'tab-ask',
                display: 'flex'
            },
            copilot: {
                buttonId: 'tab-nav-copilot',
                sectionId: 'tab-copilot',
                display: 'block'
            },
            arch: {
                buttonId: 'tab-nav-arch',
                sectionId: 'tab-arch',
                display: 'block'
            },
            learn: {
                buttonId: 'tab-nav-learn',
                sectionId: 'tab-learn',
                display: 'block'
            },
            examples: {
                buttonId: 'tab-nav-examples',
                sectionId: 'tab-examples',
                display: 'block'
            },
            settings: {
                buttonId: 'tab-nav-settings',
                sectionId: 'tab-settings',
                display: 'block'
            }
        };

        const TabNavigationController = {
            _initialized: false,
            _activeTab: 'ask',
            vscode: null,

            normalizeTab(rawName) {
                if (!rawName) return 'ask';
                let t = String(rawName).toLowerCase().trim();
                if (t.startsWith('tab-')) t = t.substring(4);
                if (t === 'learning') return 'learn';
                if (t === 'architecture') return 'arch';
                if (t === 'inspector') return 'copilot';
                if (t === 'chat' || t === 'agent') return 'ask';
                if (t === 'example') return 'examples';
                if (t === 'setting') return 'settings';
                return TAB_CONFIG[t] ? t : 'ask';
            },

            init(vscodeApi) {
                this.vscode = vscodeApi;
                if (this._initialized) return;
                this._initialized = true;

                // Restore persisted state safely
                let initialTab = 'ask';
                try {
                    if (this.vscode && this.vscode.getState) {
                        const s = this.vscode.getState();
                        if (s && s.activeTab) {
                            initialTab = this.normalizeTab(s.activeTab);
                        }
                    }
                } catch (err) {}

                this.switchTo(initialTab);
            },

            switchTo(rawName) {
                const targetTab = this.normalizeTab(rawName);
                this._activeTab = targetTab;

                // 1. Update tab buttons
                document.querySelectorAll('.tab-btn').forEach(btn => {
                    const tab = btn.getAttribute('data-tab');
                    const isActive = (tab === targetTab);
                    btn.classList.toggle('active', isActive);
                    btn.setAttribute('aria-selected', isActive ? 'true' : 'false');
                });

                // 2. Update sections
                const targetSectionId = TAB_CONFIG[targetTab] ? TAB_CONFIG[targetTab].sectionId : 'tab-ask';
                document.querySelectorAll('.section').forEach(sec => {
                    const isActive = (sec.id === targetSectionId);
                    sec.classList.toggle('active', isActive);
                });

                // 3. Persist state
                try {
                    if (this.vscode && this.vscode.getState && this.vscode.setState) {
                        const s = this.vscode.getState() || { history: [] };
                        s.activeTab = targetTab;
                        this.vscode.setState(s);
                    }
                } catch (err) {}
            },

            getActiveTab() {
                return this._activeTab;
            }
        };

        window.TabNavigationController = TabNavigationController;
        window.switchTab = function(tabName) {
            TabNavigationController.switchTo(tabName);
        };
        window.openArch = function() { TabNavigationController.switchTo('arch'); };
        window.openLearning = function() { TabNavigationController.switchTo('learn'); };
        window.openExamples = function() { TabNavigationController.switchTo('examples'); };
        window.openInspector = function() { TabNavigationController.switchTo('copilot'); };
        window.openSettings = function() { TabNavigationController.switchTo('settings'); };

        // Unified deterministic click delegation for all interactive UI elements
        document.addEventListener('click', function(e) {
            // 1. Tab buttons
            const tabBtn = e.target.closest('.tab-btn');
            if (tabBtn) {
                e.preventDefault();
                const tab = tabBtn.getAttribute('data-tab');
                if (tab) TabNavigationController.switchTo(tab);
                return;
            }

            // 2. Active model pill in header
            if (e.target.closest('#active-model-pill')) {
                e.preventDefault();
                TabNavigationController.switchTo('settings');
                return;
            }

            // 3. Action buttons with data-action
            const actionEl = e.target.closest('[data-action]');
            if (actionEl) {
                const act = actionEl.getAttribute('data-action');
                e.preventDefault();
                dispatchAction(act, actionEl, e);
                return;
            }

            // 4. Suggestion chips
            const chip = e.target.closest('.chip');
            if (chip) {
                const txt = chip.getAttribute('data-chip') || chip.innerText.trim();
                if (txt && typeof askChip === 'function') {
                    e.preventDefault();
                    askChip(txt);
                    return;
                }
            }

            // 5. Mention pills
            const mentionPill = e.target.closest('.btn-mention-pill');
            if (mentionPill) {
                const tok = mentionPill.getAttribute('data-token') || mentionPill.innerText.trim();
                if (tok) {
                    e.preventDefault();
                    insertMentionToken(tok);
                    return;
                }
            }

            // 5b. File reference pill in markdown
            const filePill = e.target.closest('.file-ref-pill');
            if (filePill) {
                const targetPath = filePill.getAttribute('data-path') || filePill.getAttribute('title');
                if (targetPath) {
                    e.preventDefault();
                    openFile(targetPath);
                    return;
                }
            }

            // 6. Example clone / card
            const cloneBtn = e.target.closest('.btn-clone');
            if (cloneBtn) {
                const card = cloneBtn.closest('.example-card');
                const slug = card ? (card.getAttribute('data-slug') || card.getAttribute('data-action-slug')) : null;
                if (slug) {
                    e.preventDefault();
                    cloneExample(slug);
                    return;
                }
            }
            const exCard = e.target.closest('.example-card');
            if (exCard) {
                const slug = exCard.getAttribute('data-slug');
                if (slug) {
                    e.preventDefault();
                    cloneExample(slug);
                    return;
                }
            }
        });

        // Universal change delegation
        document.addEventListener('change', function(e) {
            const el = e.target;
            if (el.id === 'copilot-provider-select') {
                onProviderSelectChanged(el.value);
            } else if (el.id === 'cfg-active-provider') {
                onSettingsProviderChange(el.value);
            } else if (el.id === 'cfg-ollama-model') {
                onOllamaSelectChange(el.value);
            } else if (el.id === 'cfg-settings-ollama-select') {
                const val = el.value;
                const inpMo = document.getElementById('cfg-input-model');
                if (inpMo) {
                    inpMo.value = val === 'auto' ? (window.koraLastDetectedOllamaModel || 'qwen2.5-coder:7b') : val;
                }
                const headerSelect = document.getElementById('cfg-ollama-model');
                if (headerSelect) headerSelect.value = val;
            } else if (el.id === 'cfg-ollama-endpoint' || el.id === 'cfg-input-endpoint') {
                refreshOllamaModels(el.value.trim());
            }
        });

        // Initialize TabNavigationController immediately
        TabNavigationController.init(vscode);

        let isGenerating = false;
        let lastSentPrompt = '';
        let availableContextFiles = [];
        let availableContextFolders = [];
        let currentActiveEditorFileName = '';
        let mentionSelectedIndex = 0;
        let activeMentionFilter = '';

        function isCloudEndpoint(urlStr) {
            if (!urlStr || typeof urlStr !== 'string') return false;
            try {
                const raw = urlStr.trim();
                const fullUrl = (raw.startsWith('http://') || raw.startsWith('https://')) ? raw : ('http://' + raw);
                const parsed = new URL(fullUrl);
                const host = parsed.hostname.toLowerCase();
                const cloudDomains = ['azure.com', 'openai.com', 'anthropic.com', 'googleapis.com', 'github.com'];
                return cloudDomains.some(d => host === d || host.endsWith('.' + d));
            } catch (e) {
                return false;
            }
        }

        // Drag & Drop File and Folder Attachment Handler
        const dropOverlay = document.getElementById('drop-overlay');
        const attachedBar = document.getElementById('attached-files-bar');
        let currentAttachments = [];

        function renderAttachedPills() {
            if (!attachedBar) return;
            if (!currentAttachments || currentAttachments.length === 0) {
                attachedBar.style.display = 'none';
                attachedBar.innerHTML = '';
                return;
            }
            attachedBar.style.display = 'flex';
            attachedBar.innerHTML = '';

            currentAttachments.forEach((att) => {
                const pill = document.createElement('span');
                pill.className = 'attached-file-pill';
                const isFolder = att.type === 'folder' || att.isDirectory;
                const icon = isFolder ? '📁' : '📄';
                const label = isFolder 
                    ? `${icon} ${escapeHtml(att.name)} (${att.fileCount || 0} files)`
                    : `${icon} ${escapeHtml(att.name)}`;
                pill.innerHTML = `<span>${label}</span><span class="attached-file-remove" data-id="${escapeHtml(att.id)}" title="Remove attachment">×</span>`;
                pill.querySelector('.attached-file-remove').onclick = (e) => {
                    e.stopPropagation();
                    vscode.postMessage({ type: 'removeAttachment', id: att.id });
                };
                attachedBar.appendChild(pill);
            });

            if (currentAttachments.length > 1) {
                const clearBtn = document.createElement('span');
                clearBtn.className = 'attached-file-pill';
                clearBtn.style.background = 'rgba(239, 68, 68, 0.2)';
                clearBtn.style.color = '#f87171';
                clearBtn.style.cursor = 'pointer';
                clearBtn.textContent = 'Clear All';
                clearBtn.onclick = () => {
                    vscode.postMessage({ type: 'clearAttachments' });
                };
                attachedBar.appendChild(clearBtn);
            }
        }

        function triggerFilePicker() {
            vscode.postMessage({ type: 'selectFilesToAttach' });
        }

        async function readFileAsText(file) {
            return new Promise((resolve) => {
                const reader = new FileReader();
                reader.onload = () => resolve(reader.result || '');
                reader.onerror = () => resolve('');
                reader.readAsText(file.slice(0, 128 * 1024));
            });
        }

        async function traverseDirectoryEntry(dirEntry, results, maxFiles = 60) {
            if (!dirEntry || results.length >= maxFiles) return;
            const reader = dirEntry.createReader();
            const readEntries = () => new Promise(resolve => reader.readEntries(resolve, () => resolve([])));
            const entries = await readEntries();
            for (const entry of entries) {
                if (results.length >= maxFiles) break;
                if (entry.name.startsWith('.') || entry.name === 'node_modules' || entry.name === 'target') continue;
                if (entry.isFile) {
                    await new Promise(resolve => {
                        entry.file(async file => {
                            if (file.size <= 256 * 1024) {
                                const content = await readFileAsText(file);
                                results.push({
                                    name: file.name,
                                    path: entry.fullPath || file.name,
                                    size: file.size,
                                    content: content,
                                    isDirectory: false
                                });
                            }
                            resolve();
                        }, () => resolve());
                    });
                } else if (entry.isDirectory) {
                    await traverseDirectoryEntry(entry, results, maxFiles);
                }
            }
        }

        let dragCounter = 0;

        function handleDragEnter(e) {
            e.preventDefault();
            dragCounter++;
            if (e.dataTransfer) {
                e.dataTransfer.dropEffect = 'copy';
            }
            if (dropOverlay) dropOverlay.style.display = 'flex';
        }

        function handleDragOver(e) {
            e.preventDefault();
            if (e.dataTransfer) {
                e.dataTransfer.dropEffect = 'copy';
            }
            if (dropOverlay && dropOverlay.style.display === 'none') {
                dropOverlay.style.display = 'flex';
            }
        }

        function handleDragLeave(e) {
            e.preventDefault();
            dragCounter--;
            if (dragCounter <= 0 && dropOverlay) {
                dropOverlay.style.display = 'none';
                dragCounter = 0;
            }
        }

        async function processDataTransfer(dataTransfer) {
            if (!dataTransfer) return;
            const rawPaths = [];
            const droppedItems = [];

            // 1. VS Code Explorer URI list (text/uri-list)
            const uriList = dataTransfer.getData('text/uri-list');
            if (uriList) {
                const lines = uriList.split(/\r?\n/).map(l => l.trim()).filter(Boolean);
                for (const line of lines) {
                    try {
                        const decoded = decodeURIComponent(line.replace(/^file:\/\//, ''));
                        if (decoded) rawPaths.push(decoded);
                    } catch (err) {
                        rawPaths.push(line.replace(/^file:\/\//, ''));
                    }
                }
            }

            // 2. Tree explorer MIME type
            const treeData = dataTransfer.getData('application/vnd.code.tree.explorer');
            if (treeData && typeof treeData === 'string') {
                try {
                    const parsed = JSON.parse(treeData);
                    if (Array.isArray(parsed)) {
                        for (const item of parsed) {
                            if (item.resourceUri && item.resourceUri.fsPath) {
                                rawPaths.push(item.resourceUri.fsPath);
                            } else if (item.fsPath) {
                                rawPaths.push(item.fsPath);
                            }
                        }
                    }
                } catch (e) {}
            }

            // 3. Plain text
            const textPath = dataTransfer.getData('text/plain');
            if (textPath) {
                const lines = textPath.split(/\r?\n/).map(l => l.trim()).filter(Boolean);
                for (const line of lines) {
                    if (line.includes('/') || line.includes('\\') || line.includes('.')) {
                        try {
                            const decoded = decodeURIComponent(line.replace(/^file:\/\//, ''));
                            rawPaths.push(decoded);
                        } catch (err) {
                            rawPaths.push(line);
                        }
                    }
                }
            }

            // 4. HTML5 File/Directory Items API
            if (dataTransfer.items && dataTransfer.items.length > 0) {
                for (let i = 0; i < dataTransfer.items.length; i++) {
                    const item = dataTransfer.items[i];
                    if (item.kind === 'file') {
                        const file = item.getAsFile();
                        const entry = item.webkitGetAsEntry ? item.webkitGetAsEntry() : null;
                        if (file && file.path) {
                            rawPaths.push(file.path);
                        } else if (entry) {
                            if (entry.isDirectory) {
                                await traverseDirectoryEntry(entry, droppedItems);
                            } else if (entry.isFile && file) {
                                const content = await readFileAsText(file);
                                droppedItems.push({
                                    name: file.name,
                                    path: entry.fullPath || file.name,
                                    size: file.size,
                                    content: content,
                                    isDirectory: false
                                });
                            }
                        } else if (file) {
                            const content = await readFileAsText(file);
                            droppedItems.push({
                                name: file.name,
                                path: file.name,
                                size: file.size,
                                content: content,
                                isDirectory: false
                            });
                        }
                    }
                }
            } else if (dataTransfer.files && dataTransfer.files.length > 0) {
                for (let i = 0; i < dataTransfer.files.length; i++) {
                    const file = dataTransfer.files[i];
                    if (file.path) {
                        rawPaths.push(file.path);
                    } else {
                        const content = await readFileAsText(file);
                        droppedItems.push({
                            name: file.name,
                            path: file.name,
                            size: file.size,
                            content: content,
                            isDirectory: false
                        });
                    }
                }
            }

            vscode.postMessage({
                type: 'resolveDroppedAttachments',
                rawPaths: Array.from(new Set(rawPaths)),
                droppedItems: droppedItems
            });
        }

        async function handleDrop(e) {
            e.preventDefault();
            e.stopPropagation();
            dragCounter = 0;
            if (dropOverlay) dropOverlay.style.display = 'none';
            if (e.dataTransfer) {
                await processDataTransfer(e.dataTransfer);
            }
        }

        [window, document, document.body].forEach(target => {
            if (!target) return;
            target.addEventListener('dragenter', handleDragEnter);
            target.addEventListener('dragover', handleDragOver);
            target.addEventListener('dragleave', handleDragLeave);
            target.addEventListener('drop', handleDrop);
        });

        // Request available files for context mention autocomplete and auto-discover models
        vscode.postMessage({ type: 'getAvailableContextOptions' });
        vscode.postMessage({ type: 'fetchOllamaModels' });

        // Restore persisted state safely
        try {
            const state = vscode.getState() || { history: [], activeTab: 'ask', proposals: [] };
            if (state.history && Array.isArray(state.history)) {
                state.history.forEach(item => {
                    try {
                        if (item.type === 'message') appendMessage(item.role, item.text, false);
                        else if (item.type === 'proposal') renderProposalCard(item.proposal, false);
                        else if (item.type === 'plan') renderRefactorPlanCard(item.plan, false);
                    } catch (e) {}
                });
            }
            if (state.activeTab && state.activeTab !== 'ask') {
                switchTab(state.activeTab);
            }
        } catch (e) {}

        // Auto-expand textarea & mention menu listeners
        const chatInput = document.getElementById('chat-input');
        const mentionMenu = document.getElementById('mention-menu');
        const mentionList = document.getElementById('mention-items-list');

        if (chatInput) {
            chatInput.addEventListener('dragenter', handleDragEnter);
            chatInput.addEventListener('dragover', handleDragOver);
            chatInput.addEventListener('drop', handleDrop);
            chatInput.addEventListener('paste', async (e) => {
                const text = (e.clipboardData || window.clipboardData).getData('text');
                if (text && (text.startsWith('file:///') || (/^[\/\\~A-Za-z]:?[\\\/]/.test(text) && /\.(kora|kr|toml|json|md|rs|ts|js|py)$/i.test(text)))) {
                    const lines = text.split(/\r?\n/).map(l => l.trim()).filter(Boolean);
                    const paths = lines.map(l => {
                        try { return decodeURIComponent(l.replace(/^file:\/\//, '')); }
                        catch { return l.replace(/^file:\/\//, ''); }
                    });
                    vscode.postMessage({
                        type: 'resolveDroppedAttachments',
                        rawPaths: paths,
                        droppedItems: []
                    });
                }
            });

            chatInput.addEventListener('input', function() {
                this.style.height = 'auto';
                this.style.height = Math.min(this.scrollHeight, 120) + 'px';
                handleMentionInput();
            });

            chatInput.addEventListener('keydown', function(e) {
                if (mentionMenu && mentionMenu.style.display !== 'none') {
                    if (e.key === 'ArrowDown') {
                        e.preventDefault();
                        navigateMention(1);
                        return;
                    } else if (e.key === 'ArrowUp') {
                        e.preventDefault();
                        navigateMention(-1);
                        return;
                    } else if (e.key === 'Enter' || e.key === 'Tab') {
                        e.preventDefault();
                        selectCurrentMention();
                        return;
                    } else if (e.key === 'Escape') {
                        e.preventDefault();
                        hideMentionMenu();
                        return;
                    }
                }

                if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    sendPrompt();
                }
            });
        }

        const exampleSearchInput = document.getElementById('example-search-input');
        if (exampleSearchInput) {
            exampleSearchInput.addEventListener('input', function() {
                const query = this.value.toLowerCase().trim();
                const cards = document.querySelectorAll('.example-card');
                cards.forEach(card => {
                    const slug = card.getAttribute('data-slug') || '';
                    const name = card.getAttribute('data-name') || '';
                    const cat = card.getAttribute('data-category') || '';
                    const desc = card.getAttribute('data-desc') || '';
                    if (!query || slug.includes(query) || name.includes(query) || cat.includes(query) || desc.includes(query)) {
                        card.style.display = '';
                    } else {
                        card.style.display = 'none';
                    }
                });
            });
        }

        function handleMentionInput() {
            if (!chatInput) return;
            const cursorPos = chatInput.selectionStart;
            const textBefore = chatInput.value.slice(0, cursorPos);
            const atMatch = textBefore.match(/@([a-zA-Z0-9_.:\/-]*)$/);

            if (atMatch) {
                activeMentionFilter = atMatch[1].toLowerCase();
                showMentionMenu(activeMentionFilter);
            } else {
                hideMentionMenu();
            }
        }

        function updateActiveEditorButton(fileName) {
            currentActiveEditorFileName = fileName || '';
            const btn = document.getElementById('btn-attach-active');
            if (!btn) return;
            if (currentActiveEditorFileName) {
                btn.style.display = 'inline-flex';
                btn.textContent = `📎 + ${currentActiveEditorFileName}`;
                btn.title = `Attach ${currentActiveEditorFileName} as context`;
            } else {
                btn.style.display = 'none';
            }
        }

        function showMentionMenu(filter = '') {
            if (!mentionMenu || !mentionList) return;

            const baseOptions = [
                { token: '@workspace', label: '@workspace', desc: 'Full workspace architecture & layers' },
                { token: '@selection', label: '@selection', desc: 'Reference current editor code selection' },
                { token: '@problems', label: '@problems', desc: 'Reference active compiler diagnostics & errors' },
                { token: '@terminal', label: '@terminal', desc: 'Reference active terminal sessions' }
            ];

            const fileOptions = (availableContextFiles || []).map(f => ({
                token: '@file:' + f.label,
                label: f.label,
                desc: '📄 ' + f.label
            }));

            const folderOptions = (availableContextFolders || []).map(f => ({
                token: '@folder:' + f.label,
                label: f.label,
                desc: '📁 ' + f.label
            }));

            let all = [];
            const cleanFilter = (filter || '').trim().toLowerCase();

            if (cleanFilter.startsWith('file:') || cleanFilter.startsWith('file')) {
                const subFilter = cleanFilter.replace(/^file:?/, '').trim();
                all = subFilter
                    ? fileOptions.filter(f => f.label.toLowerCase().includes(subFilter) || f.token.toLowerCase().includes(subFilter))
                    : fileOptions;
            } else if (cleanFilter.startsWith('folder:') || cleanFilter.startsWith('folder')) {
                const subFilter = cleanFilter.replace(/^folder:?/, '').trim();
                all = subFilter
                    ? folderOptions.filter(f => f.label.toLowerCase().includes(subFilter) || f.token.toLowerCase().includes(subFilter))
                    : folderOptions;
            } else if (cleanFilter) {
                const matchedBase = baseOptions.filter(b => b.token.toLowerCase().includes(cleanFilter) || b.label.toLowerCase().includes(cleanFilter));
                const matchedFiles = fileOptions.filter(f => f.label.toLowerCase().includes(cleanFilter));
                const matchedFolders = folderOptions.filter(f => f.label.toLowerCase().includes(cleanFilter));
                all = matchedBase.concat(matchedFiles).concat(matchedFolders);
            } else {
                all = baseOptions.concat(fileOptions.slice(0, 15)).concat(folderOptions.slice(0, 10));
            }

            if (all.length === 0) {
                hideMentionMenu();
                return;
            }

            mentionSelectedIndex = 0;
            mentionList.innerHTML = '';
            all.slice(0, 12).forEach((item, idx) => {
                const div = document.createElement('div');
                div.className = 'mention-item' + (idx === 0 ? ' selected' : '');
                div.innerHTML = '<span class="mention-token">' + escapeHtml(item.token) + '</span><span class="mention-desc">' + escapeHtml(item.desc) + '</span>';
                div.onclick = () => {
                    applyMentionSelection(item.token);
                };
                mentionList.appendChild(div);
            });

            mentionMenu.style.display = 'block';
        }

        function hideMentionMenu() {
            if (mentionMenu) mentionMenu.style.display = 'none';
        }

        function navigateMention(delta) {
            if (!mentionList) return;
            const items = mentionList.querySelectorAll('.mention-item');
            if (items.length === 0) return;
            if (items[mentionSelectedIndex]) items[mentionSelectedIndex].classList.remove('selected');
            mentionSelectedIndex = (mentionSelectedIndex + delta + items.length) % items.length;
            if (items[mentionSelectedIndex]) {
                items[mentionSelectedIndex].classList.add('selected');
                items[mentionSelectedIndex].scrollIntoView({ block: 'nearest' });
            }
        }

        function selectCurrentMention() {
            if (!mentionList) return;
            const items = mentionList.querySelectorAll('.mention-item');
            if (items.length > 0 && items[mentionSelectedIndex]) {
                const tokenEl = items[mentionSelectedIndex].querySelector('.mention-token');
                if (tokenEl) applyMentionSelection(tokenEl.innerText);
            }
        }

        function applyMentionSelection(token) {
            if (!chatInput) return;
            const cursorPos = chatInput.selectionStart;
            const text = chatInput.value;
            const textBefore = text.slice(0, cursorPos);
            const textAfter = text.slice(cursorPos);
            const atIndex = textBefore.lastIndexOf('@');
            if (atIndex >= 0) {
                chatInput.value = textBefore.slice(0, atIndex) + token + ' ' + textAfter;
                const newPos = atIndex + token.length + 1;
                chatInput.selectionStart = chatInput.selectionEnd = newPos;
            } else {
                insertMentionToken(token);
            }
            chatInput.focus();
            hideMentionMenu();
        }

        function insertMentionToken(token) {
            if (!chatInput) return;
            const pos = chatInput.selectionStart || chatInput.value.length;
            const before = chatInput.value.slice(0, pos);
            const after = chatInput.value.slice(pos);
            const sep = before.length > 0 && !before.endsWith(' ') ? ' ' : '';
            chatInput.value = before + sep + token + ' ' + after;
            chatInput.focus();
            chatInput.selectionStart = chatInput.selectionEnd = pos + sep.length + token.length + 1;
            chatInput.style.height = 'auto';
            chatInput.style.height = Math.min(chatInput.scrollHeight, 120) + 'px';
            if (token === '@file:' || token === '@folder:') {
                showMentionMenu(token.replace(/^@/, ''));
            } else {
                hideMentionMenu();
            }
        }

        function toggleTrackerSteps() {
            const steps = document.getElementById('tracker-steps');
            const chev = document.getElementById('tracker-chevron');
            if (!steps) return;
            if (steps.style.display === 'none') {
                steps.style.display = 'flex';
                if (chev) chev.innerText = '^';
            } else {
                steps.style.display = 'none';
                if (chev) chev.innerText = 'v';
            }
        }

        function copyText(str) {
            navigator.clipboard.writeText(str);
        }

        function stopGeneration() {
            vscode.postMessage({ type: 'stopGeneration' });
            isGenerating = false;
            const btnSend = document.getElementById('btn-send');
            const btnStop = document.getElementById('btn-stop');
            if (btnSend) btnSend.disabled = false;
            if (btnStop) btnStop.style.display = 'none';
            removeTypingIndicator();
            hideAgentTracker();
            appendMessage('assistant', '*Generation cancelled by user.*', false);
        }

        function requestWorkspaceRefactor() {
            const scope = document.getElementById('chat-scope').value;
            const input = document.getElementById('chat-input');
            const customGoal = input ? input.value.trim() : '';
            if (input) input.value = '';

            showAgentTracker('PlanningRefactor', 'Analyzing workspace invariants and planning changes...');
            vscode.postMessage({
                type: 'requestWorkspaceRefactor',
                scope: scope,
                goal: customGoal || 'Auto-Fix and refactor Kora invariants'
            });
        }

        function previewDiff(filePath, proposalId) {
            vscode.postMessage({
                type: 'previewDiff',
                filePath: filePath,
                proposalId: proposalId
            });
        }

        function openFile(filePath) {
            vscode.postMessage({
                type: 'openFile',
                filePath: filePath
            });
        }

        function acceptProposal(proposalId) {
            vscode.postMessage({
                type: 'acceptProposal',
                proposalId: proposalId
            });
        }

        function rejectProposal(proposalId) {
            vscode.postMessage({
                type: 'rejectProposal',
                proposalId: proposalId
            });
        }

        function setFileStatus(proposalId, filePath, status) {
            vscode.postMessage({
                type: 'setFileStatus',
                proposalId: proposalId,
                filePath: filePath,
                status: status
            });
        }

        function smartRun() { vscode.postMessage({ type: 'smartRun' }); }
        function checkFile() { vscode.postMessage({ type: 'checkFile' }); }
        function runTests() { vscode.postMessage({ type: 'runTests' }); }
        function explainCode() { vscode.postMessage({ type: 'explainCode' }); }
        function auditFile() { vscode.postMessage({ type: 'auditFile' }); }
        function generateTests() { vscode.postMessage({ type: 'generateTests' }); }
        function cloneExample(slug) { vscode.postMessage({ type: 'cloneExample', slug: slug }); }
        const LESSON_SNIPPETS = {
            lesson1: 'model User {\n    name: String\n    age: Int\n}\n',
            lesson2: 'class UserService {\n    repo: UserRepository\n    init(repo: UserRepository) {\n        this.repo = repo\n    }\n}\n',
            lesson3: 'class WorkerActor {\n    var count: Int\n    init() { this.count = 0 }\n    fn handle(msg: String) -> Int {\n        this.count += 1\n        return this.count\n    }\n}\n',
            lesson4: 'class LoadTestSuite {\n    init() {}\n    fn run(n: Int) -> Int {\n        var total = 0\n        for i in 0..n { total += 1 }\n        return total\n    }\n    fn teardown() -> Unit {\n        print("  [Teardown] synthetic allocations purged")\n    }\n}\n'
        };

        function insertSnippet(code) {
            const finalCode = LESSON_SNIPPETS[code] || code;
            vscode.postMessage({ type: 'insertCode', code: finalCode });
        }
        function analyzeWorkspace() { vscode.postMessage({ type: 'analyzeWorkspace' }); }

        function askChip(text) {
            const input = document.getElementById('chat-input');
            if (input) {
                input.value = text;
                input.style.height = 'auto';
                input.style.height = Math.min(input.scrollHeight, 120) + 'px';
            }
            sendPrompt();
        }

        function onProviderSelectChanged(provider) {
            updateConfigBoxVisibility(provider);
            const statusIndicator = document.getElementById('engine-indicator');
            if (statusIndicator) {
                statusIndicator.innerText = provider === 'embedded' ? 'Active (Embedded)' : 'Connected: ' + provider;
            }
            const pill = document.getElementById('active-model-pill');
            if (pill) {
                if (provider === 'embedded') pill.innerText = 'Embedded';
                else if (provider === 'copilot') pill.innerText = 'Copilot';
                else if (provider === 'ollama') pill.innerText = window.koraLastDetectedOllamaModel || 'qwen2.5-coder:7b';
                else pill.innerText = provider;
            }
            if (provider === 'ollama') {
                refreshOllamaModels();
            }
            vscode.postMessage({ type: 'updateProvider', provider: provider });
        }

        function toggleConfigPanel() {
            const box = document.getElementById('provider-config-box');
            box.style.display = box.style.display === 'none' ? 'block' : 'none';
            const current = document.getElementById('copilot-provider-select').value;
            updateConfigBoxVisibility(current);
        }

        function updateConfigBoxVisibility(provider) {
            const cp = document.getElementById('cfg-copilot');
            const gh = document.getElementById('cfg-github');
            const ol = document.getElementById('cfg-ollama');
            const em = document.getElementById('cfg-embedded');
            if (cp) cp.style.display = provider === 'copilot' ? 'block' : 'none';
            if (gh) gh.style.display = provider === 'github' ? 'block' : 'none';
            if (ol) {
                ol.style.display = provider === 'ollama' ? 'block' : 'none';
                if (provider === 'ollama') refreshOllamaModels();
            }
            if (em) em.style.display = provider === 'embedded' ? 'block' : 'none';
        }

        function refreshOllamaModels(customEp) {
            let endpoint = customEp;
            if (!endpoint) {
                const olEnd = document.getElementById('cfg-ollama-endpoint');
                const inpEnd = document.getElementById('cfg-input-endpoint');
                const candidate = (olEnd && olEnd.value.trim()) || (inpEnd && inpEnd.value.trim());
                if (candidate) {
                    if (!isCloudEndpoint(candidate)) {
                        endpoint = candidate;
                    }
                }
            }
            const statusEl = document.getElementById('ollama-scan-status');
            if (statusEl) {
                statusEl.style.display = 'block';
                statusEl.style.color = '#60A5FA';
                statusEl.innerText = 'Scanning Ollama endpoints & models...';
            }
            vscode.postMessage({ type: 'fetchOllamaModels', endpoint: endpoint || '' });
        }

        function onOllamaSelectChange(val) {
            const customInput = document.getElementById('cfg-ollama-custom');
            const settingsSelect = document.getElementById('cfg-settings-ollama-select');
            if (settingsSelect && settingsSelect.value !== val) {
                settingsSelect.value = val;
            }
            if (val === 'custom') {
                if (customInput) customInput.style.display = 'block';
            } else {
                if (customInput) customInput.style.display = 'none';
                const modelInput = document.getElementById('cfg-input-model');
                if (modelInput) modelInput.value = (val === 'auto' ? (window.koraLastDetectedOllamaModel || 'qwen2.5-coder:7b') : val);
            }
        }

        function requestGitHubLogin() {
            vscode.postMessage({ type: 'requestGitHubAuth' });
        }

        function saveGitHubConfig() {
            const token = document.getElementById('cfg-gh-token').value.trim();
            const model = document.getElementById('cfg-gh-model').value;
            vscode.postMessage({
                type: 'updateGitHubConfig',
                token: token,
                model: model
            });
            document.getElementById('provider-config-box').style.display = 'none';
        }

        function saveOllamaConfig() {
            const olEnd = document.getElementById('cfg-ollama-endpoint');
            const inpEnd = document.getElementById('cfg-input-endpoint');
            const endpoint = (olEnd && olEnd.value.trim()) || (inpEnd && inpEnd.value.trim()) || 'http://localhost:11434';
            const selectVal = document.getElementById('cfg-ollama-model').value;
            const customInput = document.getElementById('cfg-ollama-custom');
            const customVal = customInput ? customInput.value.trim() : '';
            const model = (selectVal === 'custom' && customVal) ? customVal : (selectVal !== 'custom' ? selectVal : 'qwen2.5-coder:7b');
            vscode.postMessage({
                type: 'updateOllamaConfig',
                endpoint: endpoint,
                model: model
            });
            document.getElementById('provider-config-box').style.display = 'none';
        }

        function togglePasswordVisibility(id) {
            const input = document.getElementById(id);
            if (input) input.type = input.type === 'password' ? 'text' : 'password';
        }

        function onSettingsProviderChange(val) {
            const modelInput = document.getElementById('cfg-input-model');
            const endInput = document.getElementById('cfg-input-endpoint');
            const cpGroup = document.getElementById('cfg-field-copilot');
            const olGroup = document.getElementById('cfg-field-ollama');
            const ghGroup = document.getElementById('cfg-field-github');
            const oaGroup = document.getElementById('cfg-field-openai');
            const geGroup = document.getElementById('cfg-field-gemini');
            const anGroup = document.getElementById('cfg-field-anthropic');

            if (cpGroup) cpGroup.style.display = val === 'copilot' ? 'block' : 'none';
            if (olGroup) olGroup.style.display = val === 'ollama' ? 'block' : 'none';
            if (ghGroup) ghGroup.style.display = val === 'github' ? 'block' : 'none';
            if (oaGroup) oaGroup.style.display = (val === 'openai' || val === 'custom') ? 'block' : 'none';
            if (geGroup) geGroup.style.display = val === 'gemini' ? 'block' : 'none';
            if (anGroup) anGroup.style.display = val === 'anthropic' ? 'block' : 'none';

            if (val === 'copilot') {
                if (modelInput) modelInput.value = 'default';
                if (endInput) endInput.value = '';
            } else if (val === 'github') {
                if (modelInput && (!modelInput.value || modelInput.value === 'default' || modelInput.value.includes('gemma'))) modelInput.value = 'gpt-4o-mini';
            } else if (val === 'ollama') {
                if (modelInput && (!modelInput.value || modelInput.value === 'default' || modelInput.value.includes('gpt') || modelInput.value === 'gemma:2b')) {
                    modelInput.value = window.koraLastDetectedOllamaModel || 'qwen2.5-coder:7b';
                }
                if (endInput) {
                    if (!endInput.value || isCloudEndpoint(endInput.value)) {
                        endInput.value = 'http://localhost:11434';
                    }
                }
                refreshOllamaModels(endInput ? endInput.value.trim() : null);
            } else if (val === 'gemini') {
                if (modelInput && (!modelInput.value || modelInput.value === 'default')) modelInput.value = 'gemini-1.5-flash';
            } else if (val === 'anthropic') {
                if (modelInput && (!modelInput.value || modelInput.value === 'default')) modelInput.value = 'claude-3-5-sonnet-20241022';
            }
        }

        function saveAllSettings() {
            const settings = {
                provider: document.getElementById('cfg-active-provider').value,
                githubToken: document.getElementById('cfg-input-github').value.trim(),
                openaiKey: document.getElementById('cfg-input-openai').value.trim(),
                geminiKey: document.getElementById('cfg-input-gemini').value.trim(),
                anthropicKey: document.getElementById('cfg-input-anthropic').value.trim(),
                model: document.getElementById('cfg-input-model').value.trim(),
                endpoint: document.getElementById('cfg-input-endpoint').value.trim()
            };
            vscode.postMessage({
                type: 'saveAllSettings',
                settings: settings
            });
        }

        function testConnection() {
            const statusDiv = document.getElementById('test-connection-status');
            statusDiv.style.display = 'block';
            statusDiv.style.background = 'var(--vscode-editorWidget-background)';
            statusDiv.style.border = '1px solid var(--vscode-widget-border)';
            statusDiv.style.color = 'var(--vscode-foreground)';
            statusDiv.innerText = 'Testing connection...';

            const settings = {
                provider: document.getElementById('cfg-active-provider').value,
                githubToken: document.getElementById('cfg-input-github').value.trim(),
                openaiKey: document.getElementById('cfg-input-openai').value.trim(),
                geminiKey: document.getElementById('cfg-input-gemini').value.trim(),
                anthropicKey: document.getElementById('cfg-input-anthropic').value.trim(),
                model: document.getElementById('cfg-input-model').value.trim(),
                endpoint: document.getElementById('cfg-input-endpoint').value.trim()
            };
            vscode.postMessage({
                type: 'testConnection',
                settings: settings
            });
        }

        function sendPrompt() {
            const input = document.getElementById('chat-input');
            let prompt = input.value.trim();
            if ((!prompt && currentAttachments.length === 0) || isGenerating) return;

            if (currentAttachments.length > 0) {
                const attachmentTokens = currentAttachments.map(att => {
                    const isFolder = att.type === 'folder' || att.isDirectory;
                    return isFolder ? (`@folder:${att.path || att.name}`) : (`@file:${att.path || att.name}`);
                }).join(' ');
                if (!prompt) {
                    prompt = `Analyze attached components: ${attachmentTokens}`;
                } else {
                    prompt = prompt + ' ' + attachmentTokens;
                }
            }

            lastSentPrompt = prompt;
            isGenerating = true;

            const btnSend = document.getElementById('btn-send');
            const btnStop = document.getElementById('btn-stop');
            if (btnSend) btnSend.disabled = true;
            if (btnStop) btnStop.style.display = 'inline-flex';

            const scope = document.getElementById('chat-scope').value;
            appendMessage('user', prompt);
            showTypingIndicator();
            showAgentTracker('Understanding', 'Analyzing user request with workspace context...');

            vscode.postMessage({
                type: 'askQuestion',
                prompt: prompt,
                scope: scope
            });

            input.value = '';
            input.style.height = '40px';
        }

        function appendMessage(role, text, persist = true) {
            const history = document.getElementById('chat-history');
            const scrollArea = document.getElementById('chat-scroll-area');
            const msg = document.createElement('div');
            msg.className = role === 'user' ? 'msg-user' : 'msg-assistant';

            if (role === 'assistant') {
                renderMarkdownWithCodeActions(text, msg);
            } else {
                msg.innerText = text;
            }

            history.appendChild(msg);
            if (scrollArea) scrollArea.scrollTop = scrollArea.scrollHeight;

            if (persist) {
                const s = vscode.getState() || { history: [] };
                s.history = s.history || [];
                s.history.push({ type: 'message', role, text });
                vscode.setState(s);
            }
        }

        function showTypingIndicator() {
            removeTypingIndicator();
            const history = document.getElementById('chat-history');
            const scrollArea = document.getElementById('chat-scroll-area');
            const ind = document.createElement('div');
            ind.id = 'active-typing-indicator';
            ind.className = 'typing-indicator';
            ind.innerHTML = '<span>Kora AI is reasoning...</span><span class="typing-cursor"></span>';
            history.appendChild(ind);
            if (scrollArea) scrollArea.scrollTop = scrollArea.scrollHeight;
        }

        function removeTypingIndicator() {
            const ind = document.getElementById('active-typing-indicator');
            if (ind) ind.remove();
        }

        function showAgentTracker(state, detail) {
            const tracker = document.getElementById('agent-tracker');
            if (!tracker) return;
            tracker.style.display = 'flex';
            document.getElementById('tracker-state-title').innerText = state;
            document.getElementById('tracker-detail').innerText = detail;
        }

        function hideAgentTracker() {
            const tracker = document.getElementById('agent-tracker');
            if (tracker) tracker.style.display = 'none';
        }

        function escapeHtml(str) {
            if (!str) return '';
            return String(str)
                .replace(/&/g, '&amp;')
                .replace(/</g, '&lt;')
                .replace(/>/g, '&gt;')
                .replace(/"/g, '&quot;')
                .replace(/'/g, '&#039;');
        }

        function generateInlineDiffHtml(origText, newText) {
            if (!origText && !newText) {
                return '<div class="diff-code-wrapper"><div class="diff-line diff-line-ctx">No content</div></div>';
            }
            if (!origText && newText) {
                return '<div class="diff-code-wrapper">' +
                    newText.split(/\r?\n/).map(l => '<div class="diff-line diff-line-add"><span class="diff-marker">+</span> ' + (escapeHtml(l) || '&nbsp;') + '</div>').join('') +
                    '</div>';
            }
            if (origText === newText) {
                return '<div class="diff-code-wrapper"><div class="diff-line diff-line-ctx">No text differences found between original and proposed content.</div></div>';
            }

            const origLines = (origText || '').split(/\r?\n/);
            const newLines = (newText || '').split(/\r?\n/);

            const maxLines = 400;
            const oSample = origLines.slice(0, maxLines);
            const nSample = newLines.slice(0, maxLines);

            const dp = Array.from({ length: oSample.length + 1 }, () => new Int32Array(nSample.length + 1));
            for (let i = 0; i < oSample.length; i++) {
                for (let j = 0; j < nSample.length; j++) {
                    if (oSample[i] === nSample[j]) {
                        dp[i + 1][j + 1] = dp[i][j] + 1;
                    } else {
                        dp[i + 1][j + 1] = Math.max(dp[i + 1][j], dp[i][j + 1]);
                    }
                }
            }

            const diffLines = [];
            let i = oSample.length;
            let j = nSample.length;
            while (i > 0 || j > 0) {
                if (i > 0 && j > 0 && oSample[i - 1] === nSample[j - 1]) {
                    diffLines.push({ type: 'ctx', text: oSample[i - 1] });
                    i--;
                    j--;
                } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
                    diffLines.push({ type: 'add', text: nSample[j - 1] });
                    j--;
                } else if (i > 0) {
                    diffLines.push({ type: 'del', text: oSample[i - 1] });
                    i--;
                }
            }
            diffLines.reverse();

            let html = '<div class="diff-code-wrapper">';
            for (const item of diffLines) {
                const esc = escapeHtml(item.text);
                if (item.type === 'add') {
                    html += '<div class="diff-line diff-line-add"><span class="diff-marker">+</span> ' + (esc || '&nbsp;') + '</div>';
                } else if (item.type === 'del') {
                    html += '<div class="diff-line diff-line-del"><span class="diff-marker">-</span> ' + (esc || '&nbsp;') + '</div>';
                } else {
                    html += '<div class="diff-line diff-line-ctx"><span class="diff-marker"> </span> ' + (esc || '&nbsp;') + '</div>';
                }
            }
            if (origLines.length > maxLines || newLines.length > maxLines) {
                html += '<div class="diff-line diff-line-ctx" style="opacity: 0.6;">... (truncated) Click Diff for full comparison</div>';
            }
            html += '</div>';
            return html;
        }

        function startNewChat() {
            const history = document.getElementById('chat-history');
            if (history) history.innerHTML = '';
            removeTypingIndicator();
            hideAgentTracker();
            const chatInput = document.getElementById('chat-input');
            if (chatInput) {
                chatInput.value = '';
                chatInput.style.height = 'auto';
            }
            if (typeof attachedFiles !== 'undefined' && attachedFiles.clear) {
                attachedFiles.clear();
                if (typeof updateAttachedFilesBar === 'function') updateAttachedFilesBar();
            }
            const scopeSelect = document.getElementById('chat-scope');
            if (scopeSelect) scopeSelect.value = 'workspace';

            const s = vscode.getState() || {};
            s.history = [];
            s.proposals = [];
            vscode.setState(s);

            vscode.postMessage({ type: 'newChat' });
        }

        function renderRefactorPlanCard(plan, persist = true) {
            removeTypingIndicator();
            hideAgentTracker();

            const history = document.getElementById('chat-history');
            const scrollArea = document.getElementById('chat-scroll-area');

            const card = document.createElement('div');
            card.className = 'proposal-card plan-card';
            card.id = 'plan-' + plan.id;

            if (!plan.steps || plan.steps.length === 0) {
                card.innerHTML = '<div class="proposal-card-header">' +
                    '<span class="proposal-title" style="color: #10b981;">✅ 100% Rule Compliance</span>' +
                    '<span class="badge" style="background: rgba(16, 185, 129, 0.2); color: #10b981;">0 violations</span>' +
                '</div>' +
                '<div style="font-size: 11px; margin-bottom: 6px; line-height: 1.5;">' +
                    '<b>Goal:</b> ' + escapeHtml(plan.goal) + '<br/>' +
                    '<span style="opacity: 0.9;">' + escapeHtml(plan.understanding) + '</span>' +
                '</div>' +
                '<div style="font-size: 10.5px; opacity: 0.85; padding: 4px; color: #10b981;">' +
                    'No code modifications required or proposed.' +
                '</div>';
            } else {
                let stepRows = '';
                for (const s of plan.steps) {
                    stepRows += '<div class="plan-file-item">' +
                        '<div style="font-weight: 600; color: #38bdf8; margin-bottom: 2px;">' + escapeHtml(s.fileName) + '</div>' +
                        '<ul style="margin: 0; padding-left: 16px; opacity: 0.9; font-size: 10px;">' +
                            s.plannedChanges.map(c => '<li>' + escapeHtml(c) + '</li>').join('') +
                        '</ul>' +
                    '</div>';
                }

                const actionButtons = '<div class="proposal-global-actions">' +
                    '<button class="btn-accept-all" data-action="executeRefactorPlan" data-plan-id="' + escapeHtml(plan.id) + '" data-goal="' + escapeHtml(plan.goal) + '" data-target-path="' + escapeHtml(plan.targetPath || '') + '">Approve & Execute Plan</button>' +
                    '<button class="btn-reject-all" data-action="cancelRefactorPlan" data-plan-id="' + escapeHtml(plan.id) + '">Cancel Plan</button>' +
                '</div>';

                card.innerHTML = '<div class="proposal-card-header">' +
                    '<span class="proposal-title" style="color: #60a5fa;">Proposed Refactoring Plan</span>' +
                    '<span class="badge" style="background: rgba(96, 165, 250, 0.2); color: #60a5fa;">' + plan.filesCount + ' files planned</span>' +
                '</div>' +
                '<div style="font-size: 11px; margin-bottom: 6px; line-height: 1.4;">' +
                    '<b>Goal:</b> ' + escapeHtml(plan.goal) + '<br/>' +
                    '<span style="opacity: 0.85;">' + escapeHtml(plan.understanding) + '</span>' +
                '</div>' +
                '<div style="font-size: 10px; font-weight: 600; opacity: 0.8; margin-bottom: 4px;">Planned Invariant Adjustments (Requires approval before changing code):</div>' +
                '<div class="plan-steps-list" style="margin-bottom: 8px;">' + stepRows + '</div>' +
                actionButtons;
            }

            const existingCard = document.getElementById('plan-' + plan.id);
            if (existingCard) {
                existingCard.replaceWith(card);
            } else {
                history.appendChild(card);
            }
            if (scrollArea) scrollArea.scrollTop = scrollArea.scrollHeight;

            if (persist) {
                const s = vscode.getState() || { history: [] };
                s.history = s.history || [];
                const existingIndex = s.history.findIndex(item => item.type === 'plan' && item.plan && item.plan.id === plan.id);
                if (existingIndex >= 0) {
                    s.history[existingIndex] = { type: 'plan', plan };
                } else {
                    s.history.push({ type: 'plan', plan });
                }
                vscode.setState(s);
            }
        }

        function renderProposalCard(proposal, persist = true) {
            removeTypingIndicator();
            hideAgentTracker();

            const history = document.getElementById('chat-history');
            const scrollArea = document.getElementById('chat-scroll-area');

            const card = document.createElement('div');
            card.className = 'proposal-card';
            card.id = 'card-' + proposal.id;

            if (!proposal.files || proposal.files.length === 0) {
                card.innerHTML = '<div class="proposal-card-header">' +
                    '<span class="proposal-title" style="color: #10b981;">✅ 100% Clean & Rule-Compliant</span>' +
                    '<span class="badge" style="background: rgba(16, 185, 129, 0.2); color: #10b981;">0 violations</span>' +
                '</div>' +
                '<div style="font-size: 11px; opacity: 0.9; line-height: 1.5; padding: 4px 0;">' +
                    'All inspected components strictly adhere to Kora Clean Architecture, Single-Null semantics, and compiler rules. <b>No changes required or performed.</b>' +
                '</div>';
            } else {
                let fileRows = '';
                for (let idx = 0; idx < (proposal.files || []).length; idx++) {
                    const f = proposal.files[idx];
                    const statusColor = f.status === 'accepted' ? '#10b981' : (f.status === 'rejected' ? '#ef4444' : '#38bdf8');
                    const diffTargetId = 'inline-diff-' + proposal.id + '-' + idx;
                    const diffContentHtml = generateInlineDiffHtml(f.originalContent, f.proposedContent || f.codePreview);

                    fileRows += '<div class="proposal-file-row">' +
                        '<span class="proposal-file-name" data-action="openFile" data-filepath="' + escapeHtml(f.filePath) + '" title="Click to open ' + escapeHtml(f.fileName) + '">' + escapeHtml(f.fileName) + '</span>' +
                        '<span class="diff-stat-add">+' + f.additions + '</span>' +
                        '<span class="diff-stat-del">-' + f.deletions + '</span>' +
                        '<span style="font-size: 9.5px; color:' + statusColor + '; font-weight: 600;">' + escapeHtml(f.status) + '</span>' +
                        '<button class="btn-prop-sm" data-action="previewDiff" data-filepath="' + escapeHtml(f.filePath) + '" data-proposal-id="' + escapeHtml(proposal.id) + '" title="Open native side-by-side diff in editor">Diff</button>' +
                        '<button class="btn-prop-sm" data-action="toggleInlineDiff" data-target="' + diffTargetId + '" title="Inspect proposed changes inline">View</button>' +
                        '<button class="btn-prop-sm" style="color: #10b981;" data-action="setFileStatus" data-proposal-id="' + escapeHtml(proposal.id) + '" data-filepath="' + escapeHtml(f.filePath) + '" data-status="accepted" title="Accept this file">Accept</button>' +
                        '<button class="btn-prop-sm" style="color: #ef4444;" data-action="setFileStatus" data-proposal-id="' + escapeHtml(proposal.id) + '" data-filepath="' + escapeHtml(f.filePath) + '" data-status="rejected" title="Reject this file">Reject</button>' +
                    '</div>' +
                    '<div id="' + diffTargetId + '" class="inline-diff-container">' +
                        '<div class="inline-diff-header">' +
                            '<span>Diff for ' + escapeHtml(f.fileName) + '</span>' +
                            '<button class="btn-prop-sm" data-action="previewDiff" data-filepath="' + escapeHtml(f.filePath) + '" data-proposal-id="' + escapeHtml(proposal.id) + '">Open Native Diff</button>' +
                        '</div>' +
                        diffContentHtml +
                    '</div>';
                }

                card.innerHTML = '<div class="proposal-card-header">' +
                    '<span class="proposal-title">' + escapeHtml(proposal.title) + '</span>' +
                    '<span class="badge" style="background: rgba(56, 189, 248, 0.2); color: #38bdf8;">' + proposal.totalFiles + ' files</span>' +
                '</div>' +
                '<div style="font-size: 10.5px; opacity: 0.85; margin-bottom: 6px;">Coordinated changes proposed across multiple project files. Click <b>Diff</b> for native comparison or <b>View</b> to inspect inline.</div>' +
                '<div class="proposal-files-list">' + fileRows + '</div>' +
                '<div class="proposal-global-actions">' +
                    '<button class="btn-accept-all" data-action="acceptProposal" data-proposal-id="' + escapeHtml(proposal.id) + '">Accept All Changes</button>' +
                    '<button class="btn-reject-all" data-action="rejectProposal" data-proposal-id="' + escapeHtml(proposal.id) + '">Reject All</button>' +
                '</div>';
            }

            const existingCard = document.getElementById('card-' + proposal.id);
            if (existingCard) {
                existingCard.replaceWith(card);
            } else {
                history.appendChild(card);
            }
            if (scrollArea) scrollArea.scrollTop = scrollArea.scrollHeight;

            if (persist) {
                const s = vscode.getState() || { history: [] };
                s.history = s.history || [];
                const existingIndex = s.history.findIndex(item => item.type === 'proposal' && item.proposal && item.proposal.id === proposal.id);
                if (existingIndex >= 0) {
                    s.history[existingIndex] = { type: 'proposal', proposal };
                } else {
                    s.history.push({ type: 'proposal', proposal });
                }
                vscode.setState(s);
            }
        }

        function renderMarkdownWithCodeActions(text, container) {
            container.innerHTML = '';
            const codeBlockRegex = new RegExp('\\x60{3}([a-zA-Z0-9_-]*)\\n([\\s\\S]*?)\\x60{3}', 'g');
            let lastIndex = 0;
            let match;

            while ((match = codeBlockRegex.exec(text)) !== null) {
                const preText = text.substring(lastIndex, match.index);
                if (preText.trim()) {
                    appendMarkdownText(preText, container);
                }

                const lang = (match[1] || 'kora').toLowerCase();
                const code = match[2];
                const isShell = ['bash', 'sh', 'shell', 'zsh', 'terminal', 'cmd', 'powershell'].includes(lang);

                const card = document.createElement('div');
                card.style.margin = '8px 0';
                card.style.background = 'var(--vscode-terminal-background, #181818)';
                card.style.border = '1px solid rgba(255,255,255,0.12)';
                card.style.borderRadius = '6px';
                card.style.overflow = 'hidden';

                const header = document.createElement('div');
                header.style.display = 'flex';
                header.style.justifyContent = 'space-between';
                header.style.alignItems = 'center';
                header.style.padding = '4px 8px';
                header.style.background = 'rgba(255,255,255,0.05)';
                header.style.fontSize = '10px';
                header.style.opacity = '0.85';
                header.innerHTML = '<span>' + lang.toUpperCase() + '</span>';

                const pre = document.createElement('pre');
                pre.style.margin = '0';
                pre.style.padding = '8px';
                pre.style.fontSize = '11px';
                pre.style.fontFamily = 'monospace';
                pre.style.overflowX = 'auto';
                pre.style.whiteSpace = 'pre';
                pre.innerText = code;

                const bar = document.createElement('div');
                bar.style.display = 'flex';
                bar.style.gap = '6px';
                bar.style.padding = '6px 8px';
                bar.style.background = 'rgba(0,0,0,0.2)';
                bar.style.borderTop = '1px solid rgba(255,255,255,0.08)';

                if (isShell) {
                    const btnRun = document.createElement('button');
                    btnRun.className = 'btn-prop-sm';
                    btnRun.style.background = '#3B82F6';
                    btnRun.style.color = '#fff';
                    btnRun.innerText = 'Run in Terminal';
                    btnRun.onclick = () => {
                        vscode.postMessage({ type: 'runInTerminal', command: code });
                    };
                    bar.appendChild(btnRun);
                } else {
                    const btnApply = document.createElement('button');
                    btnApply.className = 'btn-prop-sm';
                    btnApply.style.background = '#10B981';
                    btnApply.style.color = '#fff';
                    btnApply.innerText = 'Apply to File';
                    btnApply.onclick = () => {
                        vscode.postMessage({ type: 'applyFix', code: code });
                    };

                    const btnDiff = document.createElement('button');
                    btnDiff.className = 'btn-prop-sm';
                    btnDiff.innerText = 'Diff';
                    btnDiff.onclick = () => {
                        vscode.postMessage({ type: 'previewDiff', code: code });
                    };

                    bar.appendChild(btnApply);
                    bar.appendChild(btnDiff);
                }

                const btnCopy = document.createElement('button');
                btnCopy.className = 'btn-prop-sm';
                btnCopy.innerText = 'Copy';
                btnCopy.onclick = () => {
                    navigator.clipboard.writeText(code);
                    btnCopy.innerText = 'Copied';
                    setTimeout(() => { btnCopy.innerText = 'Copy'; }, 1500);
                };
                bar.appendChild(btnCopy);

                card.appendChild(header);
                card.appendChild(pre);
                card.appendChild(bar);
                container.appendChild(card);

                lastIndex = match.index + match[0].length;
            }

            const remainingText = text.substring(lastIndex);
            if (remainingText.trim()) {
                appendMarkdownText(remainingText, container);
            }
        }

        function getIconForPath(filePath) {
            if (!filePath) return (window.KORA_ICONS && window.KORA_ICONS.file) || '';
            const raw = String(filePath).toLowerCase();
            const name = raw.split(/[/\\]/).filter(Boolean).pop() || raw;
            const icons = window.KORA_ICONS || {};

            if (name.includes('.service.') || name.endsWith('service.kora') || name.endsWith('service.kr')) return icons.service || icons.file;
            if (name.includes('.model.') || name.includes('.models.') || name.endsWith('model.kora') || name.endsWith('model.kr')) return icons.model || icons.file;
            if (name.includes('.test.') || name.includes('.spec.') || name.endsWith('test.kora') || name.endsWith('test.kr') || name === 'tests' || raw.includes('/tests/')) return icons.test || icons.file;
            if (name.includes('.controller.') || name.endsWith('controller.kora')) return icons.controller || icons.file;
            if (name.includes('.repository.') || name.includes('.repo.') || name.endsWith('repository.kora')) return icons.repository || icons.file;
            if (name.includes('.routes.') || name.includes('.router.') || name.endsWith('routes.kora')) return icons.routes || icons.file;
            if (name.includes('.entity.') || name.endsWith('entity.kora')) return icons.entity || icons.file;
            if (name.includes('.dto.') || name.endsWith('dto.kora')) return icons.dto || icons.file;
            if (name.includes('.interface.') || name.endsWith('interface.kora')) return icons.interface || icons.file;
            if (name.includes('.enum.') || name.endsWith('enum.kora')) return icons.enum || icons.file;
            if (name.includes('.middleware.') || name.includes('.guard.') || name.endsWith('middleware.kora')) return icons.middleware || icons.file;
            if (name.includes('.validator.') || name.endsWith('validator.kora')) return icons.validator || icons.file;
            if (name.includes('.event.') || name.includes('.events.') || name.endsWith('event.kora')) return icons.event || icons.file;
            if (name.includes('.seeder.') || name.includes('.seed.') || name.endsWith('seeder.kora')) return icons.seeder || icons.file;
            if (name.includes('.module.') || name.endsWith('module.kora')) return icons.module || icons.file;
            if (name === 'kora.toml' || name === 'kora.lock' || name.includes('.config.')) return icons.config || icons.file;

            return icons.file || '';
        }

        function createOpenFilePill(filePath, displayLabel) {
            let rawPath = String(filePath || '').trim().replace(/^file:\/\//, '');
            rawPath = rawPath.replace(/&amp;/g, '&');

            let cleanName = displayLabel ? String(displayLabel).trim() : '';
            if (!cleanName || cleanName.includes('/') || cleanName.includes('\\') || cleanName.startsWith('file:')) {
                cleanName = rawPath.split(/[/\\]/).filter(Boolean).pop() || rawPath;
            }
            if (cleanName.endsWith('/')) {
                cleanName = cleanName.slice(0, -1);
            }
            if (rawPath.endsWith('/tests') || rawPath.endsWith('/tests/')) {
                cleanName = 'tests/';
            }

            const iconUrl = getIconForPath(rawPath);
            const safePath = escapeHtml(encodeURI(rawPath));
            const safeName = escapeHtml(cleanName);
            const iconImg = iconUrl ? `<img src="${iconUrl}" class="file-ref-icon" alt="" />` : '<span style="font-size: 11px;">📄</span>';
            return `<span class="file-ref-pill" data-path="${safePath}" title="Click to open ${safeName}"><span style="display:inline-flex; align-items:center;">${iconImg}</span><span class="file-ref-name">${safeName}</span></span>`;
        }

        function formatInlineMarkdown(str) {
            if (!str) return '';
            let s = escapeHtml(str);

            const pills = [];
            function addPill(filePath, label) {
                const idx = pills.length;
                pills.push(createOpenFilePill(filePath, label));
                return '___KORA_PILL_' + idx + '___';
            }

            // 1. Markdown file links: [label](file://path) or [label](path)
            s = s.replace(/\[([^\]]+)\]\((?:file:\/\/)?([^)]+)\)/g, (match, label, p) => {
                if (p.includes('.') || p.includes('/') || p.includes('\\')) {
                    return addPill(p, label);
                }
                return match;
            });

            // 2. Lines like "Location: /path/to/file" or "Path: /path/to/file" -> replace with file pill
            s = s.replace(/(?:Location|Path):\s*([a-zA-Z0-9_./-]+\.(?:kora|kr|toml|lock|json|rs|js|ts)|[a-zA-Z0-9_./-]+\/tests\/?|[a-zA-Z0-9_./-]+)/gi, (match, p) => {
                const trimmed = p.trim();
                if (trimmed.includes('/') && (trimmed.includes('.') || trimmed.endsWith('tests') || trimmed.endsWith('tests/'))) {
                    return addPill(trimmed);
                }
                return match;
            });

            // 3. Standalone absolute paths: /home/.../order_processor.service.kora
            s = s.replace(/(?<!["'=])(\/(?:[a-zA-Z0-9_.-]+\/)+[a-zA-Z0-9_.-]+\.(?:kora|kr|toml|lock|json|rs|js|ts)|\/(?:[a-zA-Z0-9_.-]+\/)+tests\/?)/g, (match, p) => {
                return addPill(p);
            });

            // 4. Relative paths like services/order_processor.service.kora
            s = s.replace(/(?<![a-zA-Z0-9_./-])([a-zA-Z0-9_-]+\/[a-zA-Z0-9_./-]+\.(?:kora|kr))(?![\w.-])/g, (match, p) => {
                return addPill(p);
            });

            // 5. Common kora filenames: hello.test.kora, order_processor.service.kora
            s = s.replace(/(?<![a-zA-Z0-9_./-])([a-zA-Z0-9_-]+\.(?:service|model|controller|repository|entity|dto|test|routes|seeder|validator|middleware|event|interface|enum)\.kora)(?![\w.-])/g, (match, p) => {
                return addPill(p);
            });

            // 6. Inline code containing file paths
            s = s.replace(/`([^`]+)`/g, (match, code) => {
                const trimmed = code.trim();
                if (trimmed.endsWith('.kora') || trimmed.endsWith('.kr') || trimmed.endsWith('kora.toml') || trimmed.endsWith('kora.lock')) {
                    return addPill(trimmed);
                }
                return `<code style="background: rgba(255,255,255,0.1); padding: 1px 4px; border-radius: 3px; font-family: monospace; font-size: 10.5px; color: #FBBF24;">${code}</code>`;
            });

            // 7. Styling
            s = s.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
            s = s.replace(/\*\*(.+?)\*\*/g, '<strong style="color: #F3F4F6;">$1</strong>');
            s = s.replace(/\*([^*]+)\*/g, '<em>$1</em>');

            // Restore pills safely
            for (let i = 0; i < pills.length; i++) {
                s = s.replace('___KORA_PILL_' + i + '___', pills[i]);
            }

            return s;
        }

        function renderMarkdownBlock(text) {
            const lines = text.split('\n');
            let html = '';
            let inList = false;

            for (let i = 0; i < lines.length; i++) {
                let line = lines[i];

                const hMatch = line.match(/^(#{1,4})\s+(.+)$/);
                if (hMatch) {
                    if (inList) { html += '</ul>'; inList = false; }
                    const level = hMatch[1].length;
                    const content = formatInlineMarkdown(hMatch[2]);
                    const fontSize = level === 1 ? '14px' : (level === 2 ? '13px' : '12px');
                    const color = level === 1 ? '#60A5FA' : (level === 2 ? '#34D399' : '#F3F4F6');
                    html += `<div style="font-size: ${fontSize}; font-weight: 700; color: ${color}; margin: 8px 0 4px 0;">${content}</div>`;
                    continue;
                }

                if (line.startsWith('>')) {
                    if (inList) { html += '</ul>'; inList = false; }
                    const bqText = formatInlineMarkdown(line.replace(/^>\s*/, ''));
                    html += `<div class="md-blockquote">${bqText}</div>`;
                    continue;
                }

                const listMatch = line.match(/^(\s*)[-*]\s+(.+)$/);
                if (listMatch) {
                    if (!inList) { html += '<ul style="margin: 4px 0; padding-left: 16px;">'; inList = true; }
                    html += `<li style="margin: 2px 0;">${formatInlineMarkdown(listMatch[2])}</li>`;
                    continue;
                } else if (inList) {
                    html += '</ul>';
                    inList = false;
                }

                if (!line.trim()) {
                    html += '<div style="height: 6px;"></div>';
                    continue;
                }

                html += `<div style="margin: 2px 0; line-height: 1.5;">${formatInlineMarkdown(line)}</div>`;
            }

            if (inList) { html += '</ul>'; }
            return html;
        }

        function appendMarkdownText(text, container) {
            const block = document.createElement('div');
            block.className = 'md-content-block';
            block.innerHTML = renderMarkdownBlock(text);
            container.appendChild(block);
        }

        window.addEventListener('message', event => {
            const msg = event.data;
            if (msg.type === 'agentProgress') {
                const s = msg.step;
                showAgentTracker(s.state, s.detail);
                const stepsBox = document.getElementById('tracker-steps');
                if (stepsBox && msg.allSteps) {
                    stepsBox.innerHTML = '';
                    for (const st of msg.allSteps) {
                        const line = document.createElement('div');
                        line.innerText = '[' + st.state + '] ' + st.detail;
                        stepsBox.appendChild(line);
                    }
                }
            } else if (msg.type === 'attachmentsResolved') {
                currentAttachments = msg.attachments || [];
                renderAttachedPills();
            } else if (msg.type === 'filesAttached') {
                if (msg.paths && Array.isArray(msg.paths)) {
                    vscode.postMessage({
                        type: 'resolveDroppedAttachments',
                        rawPaths: msg.paths,
                        droppedItems: []
                    });
                }
            } else if (msg.type === 'availableContextOptions') {
                availableContextFiles = msg.files || [];
                availableContextFolders = msg.folders || [];
                if (msg.activeFile) {
                    updateActiveEditorButton(msg.activeFile);
                }
            } else if (msg.type === 'activeEditorChanged') {
                updateActiveEditorButton(msg.fileName);
            } else if (msg.type === 'refactorPlanCreated') {
                isGenerating = false;
                const btnSend = document.getElementById('btn-send');
                const btnStop = document.getElementById('btn-stop');
                if (btnSend) btnSend.disabled = false;
                if (btnStop) btnStop.style.display = 'none';
                renderRefactorPlanCard(msg.plan, true);
            } else if (msg.type === 'chatCleared') {
                // Chat was cleared by startNewChat
            } else if (msg.type === 'proposalCreated') {
                isGenerating = false;
                const btnSend = document.getElementById('btn-send');
                const btnStop = document.getElementById('btn-stop');
                if (btnSend) btnSend.disabled = false;
                if (btnStop) btnStop.style.display = 'none';
                renderProposalCard(msg.proposal, true);
            } else if (msg.type === 'proposalUpdated') {
                const oldCard = document.getElementById('card-' + msg.proposal.id);
                if (oldCard) oldCard.remove();
                renderProposalCard(msg.proposal, true);
            } else if (msg.type === 'proposalApplied') {
                const card = document.getElementById('card-' + msg.proposalId);
                if (card) {
                    card.style.borderColor = '#10B981';
                    const actBox = card.querySelector('.proposal-global-actions');
                    if (actBox) actBox.innerHTML = '<div style="color: #10b981; font-weight: 600; text-align: center; grid-column: span 2;">Changes Successfully Applied!</div>';
                }
            } else if (msg.type === 'proposalRejected') {
                const card = document.getElementById('card-' + msg.proposalId);
                if (card) {
                    card.style.borderColor = '#ef4444';
                    const actBox = card.querySelector('.proposal-global-actions');
                    if (actBox) actBox.innerHTML = '<div style="color: #ef4444; font-weight: 600; text-align: center; grid-column: span 2;">Proposal Rejected.</div>';
                }
            } else if (msg.type === 'streamChunk') {
                removeTypingIndicator();
                let streamingEl = document.getElementById('active-streaming-message');
                const history = document.getElementById('chat-history');
                const scrollArea = document.getElementById('chat-scroll-area');
                if (!streamingEl) {
                    streamingEl = document.createElement('div');
                    streamingEl.id = 'active-streaming-message';
                    streamingEl.className = 'msg-assistant streaming';
                    history.appendChild(streamingEl);
                }
                renderMarkdownWithCodeActions(msg.fullText, streamingEl);
                if (scrollArea) scrollArea.scrollTop = scrollArea.scrollHeight;
            } else if (msg.type === 'assistantAnswer') {
                isGenerating = false;
                const btnSend = document.getElementById('btn-send');
                const btnStop = document.getElementById('btn-stop');
                if (btnSend) btnSend.disabled = false;
                if (btnStop) btnStop.style.display = 'none';
                removeTypingIndicator();
                hideAgentTracker();
                const streamingEl = document.getElementById('active-streaming-message');
                if (streamingEl) {
                    streamingEl.removeAttribute('id');
                    streamingEl.classList.remove('streaming');
                    renderMarkdownWithCodeActions(msg.answer, streamingEl);
                    const s = vscode.getState() || { history: [] };
                    s.history = s.history || [];
                    s.history.push({ type: 'message', role: 'assistant', text: msg.answer });
                    vscode.setState(s);
                } else {
                    appendMessage('assistant', msg.answer, true);
                }
                if (msg.provider && msg.provider !== 'embedded') {
                    document.getElementById('engine-indicator').innerText = 'Connected: ' + msg.provider;
                }
            } else if (msg.type === 'assistantError') {
                isGenerating = false;
                const btnSend = document.getElementById('btn-send');
                const btnStop = document.getElementById('btn-stop');
                if (btnSend) btnSend.disabled = false;
                if (btnStop) btnStop.style.display = 'none';
                removeTypingIndicator();
                hideAgentTracker();
                const streamingEl = document.getElementById('active-streaming-message');
                if (streamingEl) streamingEl.remove();

                // Preserve user prompt so it is not lost
                if (msg.originalPrompt && chatInput) {
                    chatInput.value = msg.originalPrompt;
                    chatInput.style.height = 'auto';
                    chatInput.style.height = Math.min(chatInput.scrollHeight, 120) + 'px';
                }

                appendMessage('assistant', '**AI Request Failed**: ' + msg.error + '\n\n*Your input was preserved below.*', false);
            } else if (msg.type === 'generationStopped') {
                isGenerating = false;
                const btnSend = document.getElementById('btn-send');
                const btnStop = document.getElementById('btn-stop');
                if (btnSend) btnSend.disabled = false;
                if (btnStop) btnStop.style.display = 'none';
                removeTypingIndicator();
                hideAgentTracker();
                const streamingEl = document.getElementById('active-streaming-message');
                if (streamingEl) {
                    streamingEl.removeAttribute('id');
                    streamingEl.classList.remove('streaming');
                }
            } else if (msg.type === 'fileUpdated') {
                const a = msg.analysis;
                if (!a) return;
                document.getElementById('active-filename').innerText = a.fileName;
                const pill = document.getElementById('active-pill');
                pill.innerText = a.classification;
                pill.className = 'pill ' + (a.isTestFile ? 'test' : (a.isRunnable ? 'runnable' : 'library'));

                document.getElementById('active-meta').innerText = a.lineCount + ' lines - ' + a.symbols.length + ' symbols - ' + a.imports.length + ' imports';
                
                const btnRun = document.getElementById('btn-run');
                if (a.isTestFile) {
                    btnRun.innerText = 'Run Test Suite (kora test)';
                } else if (a.isRunnable) {
                    btnRun.innerText = 'Run Entrypoint (kora run)';
                } else {
                    btnRun.innerText = 'Check Module (kora check)';
                }
            } else if (msg.type === 'diagnosticsUpdated') {
                const panel = document.getElementById('diagnostics-panel');
                const list = document.getElementById('diagnostics-list');
                list.innerHTML = '';
                if (msg.diagnostics && msg.diagnostics.length > 0) {
                    panel.style.display = 'block';
                    msg.diagnostics.forEach(d => {
                        const div = document.createElement('div');
                        div.className = 'diag-item';
                        div.innerText = 'Line ' + d.line + ': ' + d.message;
                        list.appendChild(div);
                    });
                } else {
                    panel.style.display = 'none';
                }
            } else if (msg.type === 'auditResult') {
                const container = document.getElementById('audit-container');
                const text = document.getElementById('audit-text');
                container.style.display = 'block';
                const audit = msg.audit || {};
                text.textContent = '';
                if (audit.passed) {
                    const okSpan = document.createElement('span');
                    okSpan.style.color = '#10B981';
                    okSpan.style.fontWeight = '600';
                    okSpan.textContent = '100% Rule Compliance: ';
                    text.appendChild(okSpan);
                    text.appendChild(document.createTextNode('No violations of .agent/rules found.'));
                } else {
                    const errSpan = document.createElement('span');
                    errSpan.style.color = '#ef4444';
                    errSpan.style.fontWeight = '600';
                    errSpan.textContent = (audit.violations || 0) + ' Rule Violation(s):';
                    text.appendChild(errSpan);
                    text.appendChild(document.createElement('br'));
                    (audit.findings || []).forEach(f => {
                        const row = document.createElement('div');
                        row.style.marginTop = '4px';
                        row.textContent = `- [${f.code || ''}] ${f.rule || ''}: ${f.message || ''} (Fix: ${f.fix || ''})`;
                        text.appendChild(row);
                    });
                }
            } else if (msg.type === 'explanationResult') {
                const container = document.getElementById('explanation-container');
                const text = document.getElementById('explanation-text');
                container.style.display = 'block';
                text.innerText = msg.explanation;
            } else if (msg.type === 'switchTab') {
                switchTab(msg.tab);
            } else if (msg.type === 'architectureResult') {
                const arch = msg.architecture;
                if (arch && arch.counts) {
                    document.getElementById('count-ctrl').innerText = arch.counts.controllers;
                    document.getElementById('count-svc').innerText = arch.counts.services;
                    document.getElementById('count-repo').innerText = arch.counts.repositories;
                    document.getElementById('count-entity').innerText = arch.counts.entities;
                    document.getElementById('count-model').innerText = arch.counts.models;
                    document.getElementById('count-test').innerText = arch.counts.tests;
                }
            } else if (msg.type === 'walkthroughResult') {
                switchTab('copilot');
                const container = document.getElementById('explanation-container');
                const text = document.getElementById('explanation-text');
                container.style.display = 'block';
                text.innerText = msg.walkthrough;
            } else if (msg.type === 'testConnectionResult') {
                const statusDiv = document.getElementById('test-connection-status');
                if (!statusDiv) return;
                statusDiv.style.display = 'block';
                if (msg.result.success) {
                    statusDiv.style.background = 'rgba(16, 185, 129, 0.15)';
                    statusDiv.style.border = '1px solid #10B981';
                    statusDiv.style.color = '#34D399';
                    statusDiv.innerText = 'Connected (' + msg.result.latencyMs + 'ms): ' + msg.result.message;
                } else {
                    statusDiv.style.background = 'rgba(239, 68, 68, 0.15)';
                    statusDiv.style.border = '1px solid #EF4444';
                    statusDiv.style.color = '#F87171';
                    let extra = '';
                    if (msg.result.recommendation) {
                        extra = '\nTip: Run ' + msg.result.recommendation;
                    }
                    statusDiv.innerText = 'Connection Error (' + msg.result.provider + '): ' + msg.result.error + extra;
                }
            } else if (msg.type === 'syncConfig') {
                const select = document.getElementById('copilot-provider-select');
                if (select) select.value = msg.provider || 'embedded';
                const pill = document.getElementById('active-model-pill');
                if (pill) {
                    if (msg.provider === 'embedded') pill.innerText = 'Embedded';
                    else if (msg.provider === 'copilot') pill.innerText = 'Copilot';
                    else if (msg.provider === 'ollama') pill.innerText = (msg.model || 'Ollama');
                    else pill.innerText = (msg.model || msg.provider);
                }
                if (msg.provider && msg.provider !== 'embedded') {
                    document.getElementById('engine-indicator').innerText = 'Connected: ' + msg.provider;
                } else {
                    document.getElementById('engine-indicator').innerText = 'Active (Embedded)';
                }
                if (msg.model) {
                    const ghModel = document.getElementById('cfg-gh-model');
                    if (ghModel && msg.provider === 'github') ghModel.value = msg.model;
                    const olModel = document.getElementById('cfg-ollama-model');
                    if (olModel && msg.provider === 'ollama') olModel.value = msg.model;
                }
                if (msg.endpoint) {
                    const olEnd = document.getElementById('cfg-ollama-endpoint');
                    if (olEnd) olEnd.value = msg.endpoint;
                }
                if (msg.settings) {
                    const s = msg.settings;
                    const actProv = document.getElementById('cfg-active-provider');
                    if (actProv && s.provider) actProv.value = s.provider;
                    onSettingsProviderChange(s.provider || msg.provider || 'embedded');
                    const inpGh = document.getElementById('cfg-input-github');
                    if (inpGh && s.githubToken) inpGh.value = s.githubToken;
                    const inpOa = document.getElementById('cfg-input-openai');
                    if (inpOa && s.openaiKey) inpOa.value = s.openaiKey;
                    const inpGe = document.getElementById('cfg-input-gemini');
                    if (inpGe && s.geminiKey) inpGe.value = s.geminiKey;
                    const inpAn = document.getElementById('cfg-input-anthropic');
                    if (inpAn && s.anthropicKey) inpAn.value = s.anthropicKey;
                    const inpMo = document.getElementById('cfg-input-model');
                    if (inpMo && s.model) inpMo.value = s.model;
                    const inpEnd = document.getElementById('cfg-input-endpoint');
                    if (inpEnd && s.endpoint) inpEnd.value = s.endpoint;
                }
                if (msg.provider === 'ollama') {
                    refreshOllamaModels();
                }
            } else if (msg.type === 'ollamaModelsLoaded') {
                const banner = document.getElementById('ollama-alert-banner');
                const bannerText = document.getElementById('ollama-alert-text');
                const pill = document.getElementById('active-model-pill');
                const select = document.getElementById('cfg-ollama-model');
                const settingsSelect = document.getElementById('cfg-settings-ollama-select');
                const statusEl = document.getElementById('ollama-scan-status');

                if (msg.activeModel) {
                    window.koraLastDetectedOllamaModel = msg.activeModel;
                }

                if (msg.endpoint) {
                    const olEnd = document.getElementById('cfg-ollama-endpoint');
                    const inpEnd = document.getElementById('cfg-input-endpoint');
                    if (olEnd) olEnd.value = msg.endpoint;
                    if (inpEnd) {
                        if (!inpEnd.value || isCloudEndpoint(inpEnd.value)) {
                            inpEnd.value = msg.endpoint;
                        }
                    }
                }

                if (msg.activeModel) {
                    const inpMo = document.getElementById('cfg-input-model');
                    if (inpMo && (!inpMo.value || inpMo.value === 'default' || inpMo.value === 'gemma:2b' || inpMo.value.includes('gpt'))) {
                        inpMo.value = msg.activeModel;
                    }
                }

                const models = msg.detailedModels || (msg.models ? msg.models.map(m => ({ name: m })) : []);

                if (statusEl) {
                    statusEl.style.display = 'block';
                    if (msg.status === 'no_models') {
                        statusEl.style.color = '#F87171';
                        statusEl.innerText = 'Connected: ' + (msg.endpoint || 'http://localhost:11434') + ' (0 models installed)';
                    } else {
                        statusEl.style.color = '#34D399';
                        statusEl.innerText = 'Connected: ' + (msg.endpoint || 'http://localhost:11434') + ' (' + models.length + ' model' + (models.length > 1 ? 's' : '') + ' available)';
                    }
                }

                if (msg.status === 'no_models') {
                    if (banner && bannerText) {
                        banner.style.display = 'block';
                        banner.style.background = 'rgba(239, 68, 68, 0.15)';
                        banner.style.borderColor = '#ef4444';
                        bannerText.textContent = '';
                        const bTag = document.createElement('b');
                        bTag.textContent = '0 Ollama models found! ';
                        bannerText.appendChild(bTag);
                        bannerText.appendChild(document.createTextNode('Run: '));
                        const codeTag = document.createElement('code');
                        codeTag.style.background = 'rgba(0,0,0,0.3)';
                        codeTag.style.padding = '2px 4px';
                        codeTag.style.borderRadius = '3px';
                        codeTag.textContent = 'ollama pull qwen2.5-coder:7b ';
                        bannerText.appendChild(codeTag);
                        const copyBtn = document.createElement('button');
                        copyBtn.className = 'btn-prop-sm';
                        copyBtn.setAttribute('data-action', 'copyText');
                        copyBtn.setAttribute('data-text', 'ollama pull qwen2.5-coder:7b');
                        copyBtn.textContent = 'Copy';
                        bannerText.appendChild(copyBtn);
                    }
                    if (pill) pill.innerText = '0 Models (Ollama)';
                } else if (msg.status === 'error') {
                    if (banner && bannerText) {
                        banner.style.display = 'block';
                        banner.style.background = 'rgba(239, 68, 68, 0.15)';
                        banner.style.borderColor = '#ef4444';
                        bannerText.textContent = '';
                        const bTag = document.createElement('b');
                        bTag.textContent = 'Ollama Error: ';
                        bannerText.appendChild(bTag);
                        const errMsg = (msg.resolution && msg.resolution.message) ? msg.resolution.message : 'Could not connect to Ollama';
                        bannerText.appendChild(document.createTextNode(errMsg));
                    }
                    if (pill) pill.innerText = 'Ollama Error';
                } else {
                    if (banner) banner.style.display = 'none';
                    if (pill && msg.activeModel) pill.innerText = msg.activeModel;
                }

                [select, settingsSelect].forEach(sel => {
                    if (!sel) return;
                    sel.innerHTML = '';
                    if (models.length === 0) {
                        const opt = document.createElement('option');
                        opt.value = 'auto';
                        opt.innerText = 'No models found (Run: ollama pull qwen2.5-coder:7b)';
                        sel.appendChild(opt);
                    } else {
                        const autoOpt = document.createElement('option');
                        autoOpt.value = 'auto';
                        autoOpt.innerText = 'Auto-select best model (' + (msg.activeModel || models[0].name) + ')';
                        sel.appendChild(autoOpt);

                        models.forEach(m => {
                            const opt = document.createElement('option');
                            opt.value = m.name;
                            const sizeStr = m.parameterSize ? (' [' + m.parameterSize + ']') : '';
                            const catStr = m.category === 'coding' ? ' [Coding]' : (m.category === 'agentic' ? ' [Agentic]' : '');
                            opt.innerText = m.name + sizeStr + catStr;
                            if (m.name === msg.activeModel) {
                                opt.selected = true;
                            }
                            sel.appendChild(opt);
                        });
                    }
                    if (sel === select) {
                        const optCustom = document.createElement('option');
                        optCustom.value = 'custom';
                        optCustom.innerText = 'Enter custom model name...';
                        sel.appendChild(optCustom);
                    }
                });
            }
        });
    
        // Expose all action functions globally so inline onclick handlers always work
        window.switchTab = switchTab;
        window.sendPrompt = sendPrompt;
        window.smartRun = smartRun;
        window.checkFile = checkFile;
        window.runTests = runTests;
        window.explainCode = explainCode;
        window.auditFile = auditFile;
        window.generateTests = generateTests;
        window.openArch = function() { switchTab('arch'); };
        window.openLearning = function() { switchTab('learn'); };
        window.openExamples = function() { switchTab('examples'); };
        window.openInspector = function() { switchTab('copilot'); };
        window.openSettings = function() { switchTab('settings'); };
        window.analyzeWorkspace = analyzeWorkspace;
        window.requestWorkspaceRefactor = requestWorkspaceRefactor;
        window.toggleConfigPanel = toggleConfigPanel;
        window.saveAllSettings = saveAllSettings;
        window.testConnection = testConnection;
        window.triggerFilePicker = triggerFilePicker;
        window.stopGeneration = stopGeneration;
        window.refreshOllamaModels = refreshOllamaModels;
        window.requestGitHubLogin = requestGitHubLogin;
        window.togglePasswordVisibility = togglePasswordVisibility;
        window.cloneExample = cloneExample;
        window.insertSnippet = insertSnippet;
        window.askChip = askChip;
        window.handleChipClick = askChip;

    
    