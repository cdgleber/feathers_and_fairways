// Fantasy Golf Application
const API_BASE = '/api';

class FantasyGolfApp {
    constructor() {
        this.selectedGolfers = new Map(); // Map<group, golferId>
        this.validatedKey = null;
        this.adminToken = localStorage.getItem('adminToken');
        this.selectedTournament = null;
        this.init();
    }

    async init() {
        this.initDarkMode();
        this.setupEventListeners();
        await this.loadInitialData();
    }

    initDarkMode() {
        const savedTheme = localStorage.getItem('theme') || 'light';
        document.documentElement.setAttribute('data-theme', savedTheme);
    }

    toggleDarkMode() {
        const current = document.documentElement.getAttribute('data-theme');
        const newTheme = current === 'dark' ? 'light' : 'dark';
        document.documentElement.setAttribute('data-theme', newTheme);
        localStorage.setItem('theme', newTheme);
    }

    setupEventListeners() {
        // Navigation
        document.querySelectorAll('.nav-btn, .mobile-nav-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const view = e.currentTarget.dataset.view;
                if (view) {
                    this.showView(view);
                    this.closeMobileNav();
                }
            });
        });

        // Dark mode toggle
        document.getElementById('darkModeToggle')?.addEventListener('click', () => {
            this.toggleDarkMode();
        });

        // Mobile menu toggle
        const mobileMenuBtn = document.getElementById('mobileMenuBtn');
        const mobileNav = document.getElementById('mobileNav');
        mobileMenuBtn?.addEventListener('click', () => {
            mobileNav.classList.toggle('active');
        });

        // Admin login
        document.getElementById('adminLoginForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.adminLogin();
        });

        // Forms
        document.getElementById('validateKeyForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.validateAccessKey();
        });

        document.getElementById('generateKeysForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.generateAccessKeys();
        });

        document.getElementById('addGolferForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.addGolfer();
        });

        document.getElementById('createTournamentForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.createTournament();
        });

        // Admin tabs
        document.querySelectorAll('#adminTabs .tab').forEach(tab => {
            tab.addEventListener('click', (e) => {
                const tabName = e.currentTarget.dataset.adminTab;
                this.switchAdminTab(tabName);
            });
        });

        // Score editor dropdowns
        document.getElementById('scoreEditorTournament')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadScoreEditorGolfers(e.target.value);
            } else {
                const golferSelect = document.getElementById('scoreEditorGolfer');
                golferSelect.innerHTML = '<option value="">Select a golfer</option>';
                golferSelect.disabled = true;
                document.getElementById('scoreEditorTable').innerHTML = '';
            }
        });
        document.getElementById('scoreEditorGolfer')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.displayScoreEditorTable(e.target.value);
            } else {
                document.getElementById('scoreEditorTable').innerHTML = '';
            }
        });

        // Team editor dropdowns
        document.getElementById('teamEditorTournament')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadTeamEditorTeams(e.target.value);
            } else {
                const teamSelect = document.getElementById('teamEditorTeam');
                teamSelect.innerHTML = '<option value="">Select a team</option>';
                teamSelect.disabled = true;
                document.getElementById('teamEditorContent').innerHTML = '';
            }
        });
        document.getElementById('teamEditorTeam')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.displayTeamEditor(e.target.value);
            } else {
                document.getElementById('teamEditorContent').innerHTML = '';
            }
        });

        // Tournament leaderboard select
        document.getElementById('tournamentSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadTournamentLeaderboard(e.target.value);
            }
        });

        // History tournament select
        document.getElementById('historyTournamentSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadHistoryTournamentData(e.target.value);
            } else {
                this.hideHistoryData();
            }
        });

        // Import file input
        document.getElementById('importFileInput')?.addEventListener('change', (e) => {
            this.previewImportFile(e);
        });

        // Import tournament select
        document.getElementById('importTournamentSelect')?.addEventListener('change', () => {
            this.updateImportCommitButton();
        });
    }

    closeMobileNav() {
        document.getElementById('mobileNav')?.classList.remove('active');
    }

    showView(viewName) {
        document.querySelectorAll('.view').forEach(view => {
            view.classList.remove('active');
        });

        const view = document.getElementById(`${viewName}View`);
        if (view) {
            view.classList.add('active');

            if (viewName === 'admin') {
                this.checkAdminAuth();
            }

            if (viewName === 'leaderboard') {
                this.loadLeaderboards();
            } else if (viewName === 'history') {
                this.loadHistoryView();
            } else if (viewName === 'admin' && this.adminToken) {
                this.loadAdminData();
            }
        }
    }

    switchAdminTab(tabName) {
        document.querySelectorAll('#adminTabs .tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.admin-tab-pane').forEach(p => p.classList.remove('active'));

        document.querySelector(`[data-admin-tab="${tabName}"]`)?.classList.add('active');
        document.getElementById(`${tabName}Tab`)?.classList.add('active');

        if (tabName === 'scoreEditor') {
            this.loadScoreEditorTournaments();
        } else if (tabName === 'teamEditor') {
            this.loadTeamEditorTournaments();
        } else if (tabName === 'import') {
            this.loadImportTournaments();
        }
    }

    async loadInitialData() {
        this.showLoading();
        try {
            await this.loadStats();
        } catch (error) {
            this.showToast('Error loading data', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadStats() {
        try {
            const [golfersRes, tournamentsRes] = await Promise.all([
                fetch(`${API_BASE}/golfers`),
                fetch(`${API_BASE}/tournaments`),
            ]);

            if (golfersRes.ok) {
                const golfers = await golfersRes.json();
                document.getElementById('totalGolfers').textContent = golfers.length;
            }

            if (tournamentsRes.ok) {
                const tournaments = await tournamentsRes.json();
                const active = tournaments.filter(t => t.is_active).length;
                document.getElementById('activeTournaments').textContent = active;

                // Count teams across all tournaments — use first active tournament if any
                const activeTournament = tournaments.find(t => t.is_active);
                if (activeTournament) {
                    const teamsRes = await fetch(`${API_BASE}/teams?tournament_id=${activeTournament.id}`);
                    if (teamsRes.ok) {
                        const teams = await teamsRes.json();
                        document.getElementById('totalTeams').textContent = teams.length;
                    }
                }
            }
        } catch (error) {
            console.error('Error loading stats:', error);
        }
    }

    async validateAccessKey() {
        const keyInput = document.getElementById('accessKey');
        const key = keyInput.value.trim().toUpperCase();

        if (!key) {
            this.showToast('Please enter an access key', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/access-keys/validate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ key_code: key })
            });

            const result = await response.json();

            if (result.valid && !result.already_used) {
                this.validatedKey = key;
                this.validatedTournamentId = result.tournament_id;
                this.showToast('Access key validated successfully!', 'success');
                document.getElementById('teamBuilderSection').classList.remove('hidden');
                await this.loadTournamentsForSelection(result.tournament_id);
            } else if (result.already_used) {
                this.validatedKey = key;
                this.validatedTournamentId = result.tournament_id;
                this.showToast('Access key already used - you can edit existing teams', 'info');
                document.getElementById('teamBuilderSection').classList.remove('hidden');
                await this.loadTournamentsForSelection(result.tournament_id);
            } else {
                this.showToast('Invalid access key', 'error');
            }
        } catch (error) {
            this.showToast('Error validating key', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadGolfersForSelection() {
        try {
            const url = this.selectedTournament
                ? `${API_BASE}/golfers/tournament/${this.selectedTournament}`
                : `${API_BASE}/golfers`;
            const response = await fetch(url);
            const golfers = await response.json();

            // Group golfers by win probability group
            const groups = {};
            golfers.forEach(golfer => {
                if (!groups[golfer.win_probability_group]) {
                    groups[golfer.win_probability_group] = [];
                }
                groups[golfer.win_probability_group].push(golfer);
            });

            const container = document.getElementById('golferGroups');
            container.innerHTML = '';

            for (let i = 1; i <= 9; i++) {
                const groupGolfers = groups[i] || [];
                const groupDiv = document.createElement('div');
                groupDiv.className = 'golfer-group';

                groupDiv.innerHTML = `
                    <div class="golfer-group-header">
                        <span class="material-icons">sports_golf</span>
                        Group ${i} ${i === 1 ? '(Highest Probability)' : i === 9 ? '(Lowest Probability)' : ''}
                    </div>
                    <div class="golfer-list">
                        ${groupGolfers.map(golfer => `
                            <div class="golfer-item">
                                <input
                                    type="radio"
                                    name="group${i}"
                                    value="${golfer.id}"
                                    id="golfer-${golfer.id}"
                                    onchange="app.selectGolfer(${i}, '${golfer.id}', '${golfer.name}')">
                                <label for="golfer-${golfer.id}">${golfer.name}${golfer.is_amateur ? ' <span class="amateur-badge">(A)</span>' : ''}</label>
                            </div>
                        `).join('')}
                    </div>
                `;

                container.appendChild(groupDiv);
            }

            this.updateSelectionStatus();
        } catch (error) {
            this.showToast('Error loading golfers', 'error');
            console.error(error);
        }
    }

    selectGolfer(group, golferId, golferName) {
        this.selectedGolfers.set(group, { id: golferId, name: golferName });
        this.updateSelectionStatus();
    }

    updateSelectionStatus() {
        const container = document.getElementById('selectionStatus');
        const createBtn = document.getElementById('createTeamBtn');

        container.innerHTML = '';
        for (let i = 1; i <= 9; i++) {
            const selected = this.selectedGolfers.get(i);
            const badge = document.createElement('div');
            badge.className = `selection-badge ${selected ? 'selected' : ''}`;
            badge.innerHTML = `
                <span>Group ${i}:</span>
                <strong>${selected ? selected.name : 'Not selected'}</strong>
            `;
            container.appendChild(badge);
        }

        createBtn.disabled = this.selectedGolfers.size !== 9;
    }

    async createTeam() {
        const playerName = document.getElementById('playerName').value.trim();
        const playerEmail = document.getElementById('playerEmail')?.value.trim() || null;

        if (!playerName) {
            this.showToast('Please enter your name', 'error');
            return;
        }

        if (this.selectedGolfers.size !== 9) {
            this.showToast('Please select one golfer from each group', 'error');
            return;
        }

        if (!this.selectedTournament) {
            this.showToast('Please select a tournament', 'error');
            return;
        }

        const golferIds = Array.from(this.selectedGolfers.values()).map(g => g.id);

        this.showLoading();
        try {
            // Check if updating existing team or creating new
            const teamsResponse = await fetch(`${API_BASE}/teams?tournament_id=${this.selectedTournament}`);
            let isUpdate = false;

            if (teamsResponse.ok) {
                const teams = await teamsResponse.json();
                const existingTeam = teams.find(t =>
                    t.tournament_id === this.selectedTournament &&
                    t.player_name === playerName
                );
                isUpdate = !!existingTeam;
            }

            const endpoint = isUpdate ? `${API_BASE}/teams/update` : `${API_BASE}/teams`;
            const payload = isUpdate ? {
                key_code: this.validatedKey,
                tournament_id: this.selectedTournament,
                golfer_ids: golferIds
            } : {
                key_code: this.validatedKey,
                player_name: playerName,
                tournament_id: this.selectedTournament,
                golfer_ids: golferIds,
                email: playerEmail
            };

            const response = await fetch(endpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });

            if (response.ok) {
                this.showToast(isUpdate ? 'Team updated successfully!' : 'Team created successfully!', 'success');
                document.getElementById('teamBuilderSection').classList.add('hidden');
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error saving team', 'error');
            }
        } catch (error) {
            this.showToast('Error saving team', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadLeaderboards() {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('tournamentSelect');

                select.innerHTML = '<option value="">Select a tournament</option>' +
                    tournaments.map(t => `
                        <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                    `).join('');

                // Auto-select active tournament
                const active = tournaments.find(t => t.is_active);
                if (active) {
                    select.value = active.id;
                    this.loadTournamentLeaderboard(active.id);
                }
            }
        } catch (error) {
            this.showToast('Error loading leaderboard', 'error');
            console.error(error);
        }
    }

    async loadTournamentLeaderboard(tournamentId) {
        this.showLoading();
        try {
            const [teamRes, scoreRes] = await Promise.all([
                fetch(`${API_BASE}/leaderboard/tournament/${tournamentId}/teams`),
                fetch(`${API_BASE}/leaderboard/tournament/${tournamentId}`)
            ]);
            const teamLeaderboard = teamRes.ok ? await teamRes.json() : [];
            const golferScores = scoreRes.ok ? await scoreRes.json() : [];
            this.displayTournamentLeaderboard(teamLeaderboard, golferScores);
        } catch (error) {
            this.showToast('Error loading tournament leaderboard', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayTournamentLeaderboard(teamLeaderboard, golferScores) {
        const container = document.getElementById('tournamentLeaderboard');

        if (teamLeaderboard.length === 0) {
            container.innerHTML = '<p class="loading">No teams registered yet.</p>';
            return;
        }

        const hasScores = golferScores.length > 0;
        const scoreMap = {};
        golferScores.forEach(s => { scoreMap[s.golfer_id] = s.total_points; });

        let html = '<div class="team-picks-grid">';
        teamLeaderboard.forEach((entry, index) => {
            const rank = index + 1;
            const golferRows = (entry.golfers || []).map(g => {
                const pts = scoreMap[g.id] !== undefined ? scoreMap[g.id] : 0;
                const ptsClass = pts > 0 ? ' positive' : pts < 0 ? ' negative' : '';
                const ptsLabel = pts > 0 ? `+${pts}` : `${pts}`;
                return `<li class="team-pick-golfer">
                    <span class="team-pick-group">G${g.win_probability_group}</span>
                    <span class="team-pick-name">${g.name}</span>
                    <span class="team-pick-pts${ptsClass}">${ptsLabel}</span>
                </li>`;
            }).join('');

            const totalHtml = hasScores
                ? `<span class="team-pick-total points">${entry.total_points > 0 ? '+' : ''}${entry.total_points} pts</span>`
                : '';

            html += `<div class="team-pick-card">
                <div class="team-pick-header">
                    <span class="rank rank-${rank}">#${rank}</span>
                    <span class="team-pick-player">${entry.player_name}</span>
                    ${totalHtml}
                </div>
                <ul class="team-pick-golfers">${golferRows}</ul>
            </div>`;
        });
        html += '</div>';
        container.innerHTML = html;
    }

    async loadAdminData() {
        await this.loadAdminStats();
        await this.loadAllTournamentsForAdmin();
    }

    // Fetch all tournaments and populate every admin dropdown that needs them
    async loadAllTournamentsForAdmin() {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (!response.ok) return;
            const tournaments = await response.json();

            const selects = [
                { id: 'keyTournamentSelect', placeholder: 'Select a tournament' },
                { id: 'fieldTournamentSelect', placeholder: 'Select a tournament' },
            ];

            selects.forEach(({ id, placeholder }) => {
                const el = document.getElementById(id);
                if (el) {
                    el.innerHTML = `<option value="">${placeholder}</option>` +
                        tournaments.map(t => `<option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>`).join('');
                }
            });
        } catch (error) {
            console.error('Error loading admin tournaments:', error);
        }
    }

    async generateAccessKeys() {
        const tournamentId = document.getElementById('keyTournamentSelect').value;
        const count = parseInt(document.getElementById('keyCount').value);

        if (!tournamentId) {
            this.showToast('Please select a tournament', 'error');
            return;
        }

        if (count < 1 || count > 50) {
            this.showToast('Please enter a number between 1 and 50', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/access-keys`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ tournament_id: tournamentId, count })
            });

            if (response.ok) {
                const keys = await response.json();
                this.displayGeneratedKeys(keys);
                this.showToast(`${count} access keys generated!`, 'success');
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error generating keys', 'error');
            }
        } catch (error) {
            this.showToast('Error generating access keys', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayGeneratedKeys(keys) {
        const container = document.getElementById('generatedKeys');
        container.innerHTML = keys.map(key => `
            <div class="key-item">
                <span class="key-code">${key.key_code}</span>
                <button class="btn btn-secondary btn-sm"
                        onclick="app.copyToClipboard('${key.key_code}')">
                    <span class="material-icons">content_copy</span>
                    Copy
                </button>
            </div>
        `).join('');
    }

    async addGolfer() {
        const name = document.getElementById('golferName').value.trim();
        const winGroup = parseInt(document.getElementById('winGroup').value);

        if (!name || !winGroup) {
            this.showToast('Please fill in all fields', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/golfers`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, win_probability_group: winGroup })
            });

            if (response.ok) {
                this.showToast('Golfer added successfully!', 'success');
                document.getElementById('addGolferForm').reset();
                await this.loadStats();
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error adding golfer', 'error');
            }
        } catch (error) {
            this.showToast('Error adding golfer', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async pasteGolfers() {
        const textarea = document.getElementById('pasteGolferNames');
        const names = textarea.value.split('\n').map(n => n.trim()).filter(n => n.length > 0);

        if (names.length === 0) {
            this.showToast('Please enter at least one golfer name', 'error');
            return;
        }

        const isAmateur = document.getElementById('pasteGolferAmateur').checked;

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/golfers/paste`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ names, is_amateur: isAmateur })
            });

            const result = await response.json();
            const resultDiv = document.getElementById('pasteGolferResult');

            if (response.ok) {
                const created = result.results.filter(r => r.created).length;
                const found = result.results.filter(r => !r.created).length;

                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${created}</strong> golfers created, <strong>${found}</strong> already existed.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">${result.errors.map(e => `<li>${e}</li>`).join('')}</ul>`;
                }
                html += `</div>`;
                resultDiv.innerHTML = html;
                resultDiv.classList.remove('hidden');
                this.showToast(`${created} golfers created!`, 'success');
                textarea.value = '';
                await this.loadStats();
            } else {
                this.showToast(result.message || 'Error importing golfers', 'error');
            }
        } catch (error) {
            this.showToast('Error importing golfers', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async createTournament() {
        const name = document.getElementById('tournamentName').value.trim();
        const startDate = document.getElementById('tournamentStart').value;
        const endDate = document.getElementById('tournamentEnd').value;

        if (!name || !startDate || !endDate) {
            this.showToast('Please fill in all fields', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/tournaments`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, start_date: startDate, end_date: endDate })
            });

            if (response.ok) {
                this.showToast('Tournament created successfully!', 'success');
                document.getElementById('createTournamentForm').reset();
                await this.loadStats();
                await this.loadAllTournamentsForAdmin();
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error creating tournament', 'error');
            }
        } catch (error) {
            this.showToast('Error creating tournament', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    // ESPN Field Fetch
    async fetchEspnField() {
        const tournamentId = document.getElementById('fieldTournamentSelect').value;
        const espnId = document.getElementById('fieldEspnId').value.trim();

        if (!tournamentId) {
            this.showToast('Please select a tournament', 'error');
            return;
        }
        if (!espnId) {
            this.showToast('Please enter an ESPN tournament ID', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/${tournamentId}/espn-field`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ espn_tournament_id: espnId })
                }
            );

            if (response.ok) {
                const data = await response.json();
                this.espnFieldTournamentId = tournamentId;
                this.espnFieldGroups = data.groups;
                this.displayEspnFieldPreview(data.groups);
                this.showToast(`Field fetched: ${data.groups.reduce((s, g) => s + g.golfers.length, 0)} golfers in 9 groups`, 'success');
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error fetching ESPN field', 'error');
            }
        } catch (error) {
            this.showToast('Error fetching ESPN field', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayEspnFieldPreview(groups) {
        const preview = document.getElementById('espnFieldPreview');
        const container = document.getElementById('espnFieldGroups');

        let html = '<div class="leaderboard"><table><thead><tr><th>Group</th><th>Golfer</th><th>ESPN ID</th><th>Status</th><th>Move to Group</th></tr></thead><tbody>';

        groups.forEach(group => {
            group.golfers.forEach(golfer => {
                html += `<tr>
                    <td><strong>G${group.group}</strong></td>
                    <td>${golfer.name}</td>
                    <td style="font-size: 0.85em; color: var(--text-secondary);">${golfer.espn_id || '—'}</td>
                    <td>${golfer.created ? '<span style="color: var(--success);">New</span>' : 'Existing'}</td>
                    <td>
                        <select class="form-input field-group-select" style="width: 80px; padding: 4px 8px;"
                            data-golfer-id="${golfer.golfer_id}" data-original-group="${group.group}">
                            ${[1,2,3,4,5,6,7,8,9].map(g => `<option value="${g}"${g === group.group ? ' selected' : ''}>${g}</option>`).join('')}
                        </select>
                    </td>
                </tr>`;
            });
        });

        html += '</tbody></table></div>';
        container.innerHTML = html;
        preview.classList.remove('hidden');
        document.getElementById('saveGroupsResult').classList.add('hidden');
    }

    async saveGroupAssignments() {
        const selects = document.querySelectorAll('.field-group-select');
        const assignments = Array.from(selects).map(sel => ({
            golfer_id: sel.dataset.golferId,
            group: parseInt(sel.value)
        }));

        if (assignments.length === 0) {
            this.showToast('No assignments to save', 'error');
            return;
        }

        const tournamentId = this.espnFieldTournamentId;
        if (!tournamentId) {
            this.showToast('No tournament selected', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/${tournamentId}/groups`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ assignments })
                }
            );

            const result = await response.json();
            const resultDiv = document.getElementById('saveGroupsResult');

            if (response.ok) {
                resultDiv.innerHTML = `<div class="info-card" style="border-left: 4px solid var(--success);"><p><strong>${result.total_processed}</strong> group assignments saved.</p></div>`;
                resultDiv.classList.remove('hidden');
                this.showToast('Group assignments saved!', 'success');
            } else {
                this.showToast(result.message || 'Error saving groups', 'error');
            }
        } catch (error) {
            this.showToast('Error saving group assignments', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    // Utility functions
    formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }

    copyToClipboard(text) {
        navigator.clipboard.writeText(text).then(() => {
            this.showToast('Copied to clipboard!', 'success');
        }).catch(() => {
            this.showToast('Failed to copy', 'error');
        });
    }

    showLoading() {
        document.getElementById('loadingOverlay')?.classList.remove('hidden');
    }

    hideLoading() {
        document.getElementById('loadingOverlay')?.classList.add('hidden');
    }

    showToast(message, type = 'info') {
        const container = document.getElementById('toastContainer');
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;

        const icon = type === 'success' ? 'check_circle' :
                     type === 'error' ? 'error' :
                     'info';

        toast.innerHTML = `
            <span class="material-icons">${icon}</span>
            <span>${message}</span>
        `;

        container.appendChild(toast);

        setTimeout(() => {
            toast.style.animation = 'slideOut 0.3s ease';
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }

    // Admin authentication
    async adminLogin() {
        const password = document.getElementById('adminPassword').value;

        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/admin/login`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ password })
            });

            const result = await response.json();

            if (result.success && result.token) {
                this.adminToken = result.token;
                localStorage.setItem('adminToken', result.token);
                this.showToast('Login successful!', 'success');
                document.getElementById('adminLoginModal').classList.add('hidden');
                document.getElementById('adminContent').classList.remove('hidden');
                this.loadAdminData();
            } else {
                this.showToast('Invalid password', 'error');
            }
        } catch (error) {
            this.showToast('Error logging in', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    checkAdminAuth() {
        const modal = document.getElementById('adminLoginModal');
        const content = document.getElementById('adminContent');

        if (this.adminToken) {
            modal.classList.add('hidden');
            content.classList.remove('hidden');
        } else {
            modal.classList.remove('hidden');
            content.classList.add('hidden');
        }
    }

    adminLogout() {
        this.adminToken = null;
        localStorage.removeItem('adminToken');
        this.showToast('Logged out', 'info');
        this.showView('home');
    }

    async makeAdminRequest(url, options = {}) {
        if (!this.adminToken) {
            this.showToast('Admin authentication required', 'error');
            throw new Error('Not authenticated');
        }

        const response = await fetch(url, {
            ...options,
            headers: {
                ...options.headers,
                'Authorization': `Bearer ${this.adminToken}`,
            }
        });

        if (response.status === 401) {
            this.adminToken = null;
            localStorage.removeItem('adminToken');
            this.showToast('Session expired. Please log in again.', 'error');
            this.checkAdminAuth();
            throw new Error('Session expired');
        }

        return response;
    }

    async loadTournamentsForSelection(preSelectedTournamentId) {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (response.ok) {
                const tournaments = await response.json();
                this.displayTournamentSelection(tournaments, preSelectedTournamentId);
            }
        } catch (error) {
            console.error('Error loading tournaments:', error);
        }
    }

    displayTournamentSelection(tournaments, preSelectedId) {
        // If we have a pre-selected tournament from the access key, filter to just that one
        // Otherwise show all upcoming tournaments
        let available;
        if (preSelectedId) {
            available = tournaments.filter(t => t.id === preSelectedId);
        } else {
            available = tournaments.filter(t => {
                const startDate = new Date(t.start_date);
                return startDate >= new Date();
            });
        }

        if (available.length === 0) {
            this.showToast('No tournaments available for this key', 'error');
            return;
        }

        const container = document.getElementById('teamBuilderSection');
        container.innerHTML = `
            <h3 class="card-title">Select Tournament</h3>
            <div class="form-group">
                <label for="tournamentSelectForTeam" class="form-label">Choose a tournament to create/edit your team</label>
                <select id="tournamentSelectForTeam" class="form-input">
                    <option value="">-- Select Tournament --</option>
                    ${available.map(t => `
                        <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                    `).join('')}
                </select>
            </div>
            <div id="teamFormContainer"></div>
        `;

        // Auto-select if only one option
        if (available.length === 1) {
            document.getElementById('tournamentSelectForTeam').value = available[0].id;
            this.selectedTournament = available[0].id;
            this.checkExistingTeam();
        } else {
            document.getElementById('tournamentSelectForTeam').addEventListener('change', (e) => {
                if (e.target.value) {
                    this.selectedTournament = e.target.value;
                    this.checkExistingTeam();
                }
            });
        }
    }

    async checkExistingTeam() {
        if (!this.validatedKey || !this.selectedTournament) return;

        try {
            const response = await fetch(`${API_BASE}/teams?tournament_id=${this.selectedTournament}`);
            if (response.ok) {
                const teams = await response.json();
                const existingTeam = teams.find(t =>
                    t.tournament_id === this.selectedTournament
                );

                if (existingTeam) {
                    this.loadExistingTeamForEdit(existingTeam.id);
                } else {
                    this.showTeamForm();
                }
            }
        } catch (error) {
            console.error('Error checking existing team:', error);
            this.showTeamForm();
        }
    }

    async loadExistingTeamForEdit(teamId) {
        try {
            const response = await fetch(`${API_BASE}/teams/${teamId}/golfers`);
            if (response.ok) {
                const golfers = await response.json();

                this.selectedGolfers.clear();
                golfers.forEach(g => {
                    this.selectedGolfers.set(g.win_probability_group, {
                        id: g.id,
                        name: g.name
                    });
                });

                this.showTeamForm(true);
                this.showToast('Editing existing team - you can update your selections', 'info');
            }
        } catch (error) {
            console.error('Error loading existing team:', error);
            this.showTeamForm();
        }
    }

    async showTeamForm(isEdit = false) {
        const container = document.getElementById('teamFormContainer');

        container.innerHTML = `
            <form id="createTeamForm" class="form" style="margin-top: 20px;">
                <div class="form-group">
                    <label for="playerName" class="form-label">Your Name</label>
                    <input
                        type="text"
                        id="playerName"
                        class="form-input"
                        placeholder="Enter your name"
                        required>
                </div>

                <div class="form-group">
                    <label for="playerEmail" class="form-label">Email (optional)</label>
                    <input
                        type="email"
                        id="playerEmail"
                        class="form-input"
                        placeholder="Enter your email address">
                </div>

                <div class="selection-info">
                    <p><strong>Select 9 golfers - one from each skill group</strong></p>
                    <div id="selectionStatus" class="selection-status"></div>
                </div>

                <div id="golferGroups" class="golfer-groups"></div>

                <button type="submit" class="btn btn-primary" id="createTeamBtn" disabled>
                    <span class="material-icons">${isEdit ? 'edit' : 'add_circle'}</span>
                    ${isEdit ? 'Update Team' : 'Create Team'}
                </button>
            </form>
        `;

        document.getElementById('createTeamForm').addEventListener('submit', (e) => {
            e.preventDefault();
            this.createTeam();
        });

        await this.loadGolfersForSelection();
    }

    // Score Editor methods
    async loadScoreEditorTournaments() {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('scoreEditorTournament');
                select.innerHTML = '<option value="">Select a tournament</option>' +
                    tournaments.map(t => `<option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>`).join('');
            }
        } catch (error) {
            console.error('Error loading score editor tournaments:', error);
        }
    }

    async loadScoreEditorGolfers(tournamentId) {
        try {
            const [scoresRes, golfersRes] = await Promise.all([
                fetch(`${API_BASE}/scores/tournament/${tournamentId}`),
                fetch(`${API_BASE}/golfers/tournament/${tournamentId}`)
            ]);

            if (scoresRes.ok && golfersRes.ok) {
                this.scoreEditorScores = await scoresRes.json();
                this.scoreEditorGolfers = await golfersRes.json();
                this.scoreEditorTournamentId = tournamentId;

                const golferIdsWithScores = new Set(this.scoreEditorScores.map(s => s.golfer_id));
                const golfersWithScores = this.scoreEditorGolfers.filter(g => golferIdsWithScores.has(g.id));

                const select = document.getElementById('scoreEditorGolfer');
                select.disabled = false;
                select.innerHTML = '<option value="">Select a golfer</option>' +
                    golfersWithScores.map(g => `<option value="${g.id}">${g.name} (Group ${g.win_probability_group})</option>`).join('');
                document.getElementById('scoreEditorTable').innerHTML = '';
            }
        } catch (error) {
            this.showToast('Error loading score data', 'error');
            console.error(error);
        }
    }

    displayScoreEditorTable(golferId) {
        const scores = this.scoreEditorScores.filter(s => s.golfer_id === golferId);
        const container = document.getElementById('scoreEditorTable');

        if (scores.length === 0) {
            container.innerHTML = '<p class="loading">No scores found for this golfer.</p>';
            return;
        }

        const days = [...new Set(scores.map(s => s.day))].sort((a, b) => a - b);

        let html = '<div class="score-editor-table"><table><thead><tr><th>Hole</th>';
        days.forEach(d => { html += `<th colspan="2">Round ${d}</th>`; });
        html += '</tr><tr><th></th>';
        days.forEach(() => { html += '<th>Strokes</th><th>To Par</th>'; });
        html += '</tr></thead><tbody>';

        for (let hole = 1; hole <= 18; hole++) {
            html += `<tr><td><strong>${hole}</strong></td>`;
            days.forEach(day => {
                const score = scores.find(s => s.day === day && s.hole === hole);
                const strokes = score ? score.strokes : '';
                const scoreToPar = score ? score.score_to_par : '';
                html += `<td><input type="number" class="score-input" min="1" max="15"
                    data-golfer-id="${golferId}" data-day="${day}" data-hole="${hole}" data-field="strokes"
                    data-original="${strokes}" value="${strokes}"
                    onchange="app.markScoreChanged(this)"></td>`;
                html += `<td><input type="number" class="score-input" min="-5" max="10"
                    data-golfer-id="${golferId}" data-day="${day}" data-hole="${hole}" data-field="score_to_par"
                    data-original="${scoreToPar}" value="${scoreToPar}"
                    onchange="app.markScoreChanged(this)"></td>`;
            });
            html += '</tr>';
        }

        html += '</tbody></table></div>';
        html += '<button class="btn btn-primary" style="margin-top: var(--spacing-md);" onclick="app.saveEditedScores()"><span class="material-icons">save</span> Save Changes</button>';

        container.innerHTML = html;
    }

    markScoreChanged(input) {
        if (input.value !== input.dataset.original) {
            input.classList.add('changed');
        } else {
            input.classList.remove('changed');
        }
    }

    async saveEditedScores() {
        const changedInputs = document.querySelectorAll('#scoreEditorTable .score-input.changed[data-field="strokes"]');
        if (changedInputs.length === 0) {
            this.showToast('No changes to save', 'info');
            return;
        }

        const scores = [];
        changedInputs.forEach(input => {
            const golferId = input.dataset.golferId;
            const day = parseInt(input.dataset.day);
            const hole = parseInt(input.dataset.hole);
            const strokes = parseInt(input.value);

            const parInput = document.querySelector(
                `.score-input[data-golfer-id="${golferId}"][data-day="${day}"][data-hole="${hole}"][data-field="score_to_par"]`
            );
            const scoreToPar = parseInt(parInput.value);

            scores.push({ golfer_id: golferId, day, hole, strokes, score_to_par: scoreToPar });
        });

        const changedParInputs = document.querySelectorAll('#scoreEditorTable .score-input.changed[data-field="score_to_par"]');
        changedParInputs.forEach(input => {
            const golferId = input.dataset.golferId;
            const day = parseInt(input.dataset.day);
            const hole = parseInt(input.dataset.hole);

            if (scores.find(s => s.golfer_id === golferId && s.day === day && s.hole === hole)) return;

            const strokesInput = document.querySelector(
                `.score-input[data-golfer-id="${golferId}"][data-day="${day}"][data-hole="${hole}"][data-field="strokes"]`
            );
            scores.push({
                golfer_id: golferId,
                day,
                hole,
                strokes: parseInt(strokesInput.value),
                score_to_par: parseInt(input.value)
            });
        });

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/scores`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    tournament_id: this.scoreEditorTournamentId,
                    scores
                })
            });

            if (response.ok) {
                this.showToast('Scores saved successfully!', 'success');
                await this.loadScoreEditorGolfers(this.scoreEditorTournamentId);
                const golferSelect = document.getElementById('scoreEditorGolfer');
                if (golferSelect.value) {
                    this.displayScoreEditorTable(golferSelect.value);
                }
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error saving scores', 'error');
            }
        } catch (error) {
            this.showToast('Error saving scores', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    // Team Editor methods
    async loadTeamEditorTournaments() {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('teamEditorTournament');
                select.innerHTML = '<option value="">Select a tournament</option>' +
                    tournaments.map(t => `<option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>`).join('');
            }
        } catch (error) {
            console.error('Error loading team editor tournaments:', error);
        }
    }

    async loadTeamEditorTeams(tournamentId) {
        this.teamEditorTournamentId = tournamentId;
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/tournaments/${tournamentId}/teams`);
            if (response.ok) {
                const teams = await response.json();
                const select = document.getElementById('teamEditorTeam');
                select.disabled = false;
                select.innerHTML = '<option value="">Select a team</option>' +
                    teams.map(t => `<option value="${t.id}">${t.player_name}</option>`).join('');
                document.getElementById('teamEditorContent').innerHTML = '';
            }
        } catch (error) {
            this.showToast('Error loading teams', 'error');
            console.error(error);
        }
    }

    async displayTeamEditor(teamId) {
        this.teamEditorTeamId = teamId;
        const tournamentId = this.teamEditorTournamentId;

        this.showLoading();
        try {
            const [golferRes, teamGolferRes] = await Promise.all([
                fetch(`${API_BASE}/golfers/tournament/${tournamentId}`),
                fetch(`${API_BASE}/teams/${teamId}/golfers`)
            ]);

            if (!golferRes.ok || !teamGolferRes.ok) {
                this.showToast('Error loading team data', 'error');
                return;
            }

            const allGolfers = await golferRes.json();
            const teamGolfers = await teamGolferRes.json();

            const golfersByGroup = {};
            allGolfers.forEach(g => {
                if (!golfersByGroup[g.win_probability_group]) {
                    golfersByGroup[g.win_probability_group] = [];
                }
                golfersByGroup[g.win_probability_group].push(g);
            });

            const currentByGroup = {};
            teamGolfers.forEach(g => {
                currentByGroup[g.win_probability_group] = g;
            });

            let html = '<div class="team-editor-table"><table><thead><tr><th>Group</th><th>Current Golfer</th><th>Replacement</th></tr></thead><tbody>';

            for (let group = 1; group <= 9; group++) {
                const current = currentByGroup[group];
                const options = golfersByGroup[group] || [];

                html += `<tr><td><strong>Group ${group}</strong></td>`;
                html += `<td>${current ? current.name : '<em>None</em>'}</td>`;
                html += `<td><select class="team-editor-select" data-group="${group}">`;
                options.forEach(g => {
                    const selected = current && current.id === g.id ? ' selected' : '';
                    html += `<option value="${g.id}"${selected}>${g.name}</option>`;
                });
                html += '</select></td></tr>';
            }

            html += '</tbody></table></div>';
            html += `<button class="btn btn-primary" style="margin-top: var(--spacing-md);" onclick="app.saveTeamEditorChanges()"><span class="material-icons">save</span> Save Changes</button>`;

            document.getElementById('teamEditorContent').innerHTML = html;
        } catch (error) {
            this.showToast('Error loading team editor', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async saveTeamEditorChanges() {
        const selects = document.querySelectorAll('.team-editor-select');
        const golferIds = Array.from(selects).map(s => s.value);

        if (golferIds.length !== 9) {
            this.showToast('Must have exactly 9 golfers selected', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/teams/${this.teamEditorTeamId}/golfers`,
                {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        tournament_id: this.teamEditorTournamentId,
                        golfer_ids: golferIds
                    })
                }
            );

            if (response.ok) {
                this.showToast('Team updated successfully!', 'success');
                await this.displayTeamEditor(this.teamEditorTeamId);
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error updating team', 'error');
            }
        } catch (error) {
            this.showToast('Error updating team', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadAdminStats() {
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/stats`);
            if (response.ok) {
                const stats = await response.json();
                this.displayAdminStats(stats);
            }
        } catch (error) {
            console.error('Error loading admin stats:', error);
        }
    }

    displayAdminStats(stats) {
        const container = document.getElementById('adminStatsContent');
        if (!container) return;

        const dist = stats.score_distribution;
        const total = dist.eagles_or_better + dist.birdies + dist.pars + dist.bogeys_or_worse;

        let html = '<div class="admin-stats-grid">';
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.total_tournaments + '</span><span class="admin-stat-label">Tournaments</span></div>';
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.total_teams + '</span><span class="admin-stat-label">Teams</span></div>';
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.total_golfers + '</span><span class="admin-stat-label">Active Golfers</span></div>';
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.total_scores + '</span><span class="admin-stat-label">Hole Scores</span></div>';
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.access_keys_used + ' / ' + stats.access_keys_total + '</span><span class="admin-stat-label">Keys Used</span></div>';
        html += '</div>';

        if (total > 0) {
            html += '<h4 style="margin: var(--spacing-lg) 0 var(--spacing-md);">Score Distribution</h4>';
            html += '<div class="score-dist-grid">';
            html += '<div class="score-dist-bar"><div class="score-dist-fill score-dist-eagle" style="width: ' + (dist.eagles_or_better / total * 100).toFixed(1) + '%"></div><span class="score-dist-label">Eagles+ (' + dist.eagles_or_better + ')</span></div>';
            html += '<div class="score-dist-bar"><div class="score-dist-fill score-dist-birdie" style="width: ' + (dist.birdies / total * 100).toFixed(1) + '%"></div><span class="score-dist-label">Birdies (' + dist.birdies + ')</span></div>';
            html += '<div class="score-dist-bar"><div class="score-dist-fill score-dist-par" style="width: ' + (dist.pars / total * 100).toFixed(1) + '%"></div><span class="score-dist-label">Pars (' + dist.pars + ')</span></div>';
            html += '<div class="score-dist-bar"><div class="score-dist-fill score-dist-bogey" style="width: ' + (dist.bogeys_or_worse / total * 100).toFixed(1) + '%"></div><span class="score-dist-label">Bogeys+ (' + dist.bogeys_or_worse + ')</span></div>';
            html += '</div>';
        }

        if (stats.popular_golfers.length > 0) {
            html += '<h4 style="margin: var(--spacing-lg) 0 var(--spacing-md);">Most Selected Golfers</h4>';
            html += '<div class="leaderboard"><table><thead><tr><th>Rank</th><th>Golfer</th><th>Times Selected</th></tr></thead><tbody>';
            stats.popular_golfers.forEach(function(g, i) {
                html += '<tr><td><span class="rank rank-' + (i + 1) + '">#' + (i + 1) + '</span></td><td>' + g.golfer_name + '</td><td>' + g.times_selected + '</td></tr>';
            });
            html += '</tbody></table></div>';
        }

        container.innerHTML = html;
    }

    // History view methods
    async loadHistoryView() {
        try {
            const response = await fetch(`${API_BASE}/tournaments/completed`);
            const select = document.getElementById('historyTournamentSelect');

            if (response.ok) {
                const tournaments = await response.json();

                if (tournaments.length === 0) {
                    select.innerHTML = '<option value="">No completed tournaments</option>';
                    this.hideHistoryData();
                    document.getElementById('historyEmptyState').classList.remove('hidden');
                    document.getElementById('historyEmptyMessage').textContent = 'No completed tournaments yet.';
                    return;
                }

                select.innerHTML = tournaments.map((t, i) =>
                    `<option value="${t.id}"${i === 0 ? ' selected' : ''}>${t.name} (${this.formatDate(t.start_date)} - ${this.formatDate(t.end_date)})</option>`
                ).join('');

                // Load the first tournament automatically
                await this.loadHistoryTournamentData(tournaments[0].id);
            }
        } catch (error) {
            console.error('Error loading history view:', error);
        }
    }

    async loadHistoryTournamentData(tournamentId) {
        this.showLoading();
        try {
            const [statsRes, leaderboardRes] = await Promise.all([
                fetch(`${API_BASE}/tournaments/${tournamentId}/stats`),
                fetch(`${API_BASE}/leaderboard/tournament/${tournamentId}/teams`)
            ]);

            document.getElementById('historyEmptyState').classList.add('hidden');

            if (statsRes.ok) {
                const stats = await statsRes.json();
                this.displayHistoryStats(stats);
            }

            if (leaderboardRes.ok) {
                const leaderboard = await leaderboardRes.json();
                this.displayHistoryLeaderboard(leaderboard);
            }
        } catch (error) {
            this.showToast('Error loading tournament data', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayHistoryStats(stats) {
        const section = document.getElementById('historyStatsSection');
        const container = document.getElementById('historyStatsContent');
        section.classList.remove('hidden');

        let html = '<div class="tournament-stats-grid">';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.total_holes_played + '</span><span class="stat-item-label">Holes Played</span></div>';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.total_fantasy_points + '</span><span class="stat-item-label">Total Fantasy Points</span></div>';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.eagles_or_better + '</span><span class="stat-item-label">Eagles+</span></div>';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.birdies + '</span><span class="stat-item-label">Birdies</span></div>';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.pars + '</span><span class="stat-item-label">Pars</span></div>';
        html += '<div class="stat-item-compact"><span class="stat-item-value">' + stats.bogeys_or_worse + '</span><span class="stat-item-label">Bogeys+</span></div>';
        html += '</div>';

        if (stats.best_round_golfer) {
            html += '<p style="margin-top: var(--spacing-md); color: var(--text-secondary);">Best Round: <strong>' + stats.best_round_golfer + '</strong> (' + stats.best_round_points + ' pts)</p>';
        }

        container.innerHTML = html;
    }

    displayHistoryLeaderboard(leaderboard) {
        const section = document.getElementById('historyLeaderboardSection');
        const container = document.getElementById('historyLeaderboard');
        section.classList.remove('hidden');

        if (leaderboard.length === 0) {
            container.innerHTML = '<p class="loading">No teams found for this tournament.</p>';
            return;
        }

        let html = '<table><thead><tr><th>Rank</th><th>Player</th><th>Golfers</th><th>Points</th></tr></thead><tbody>';
        leaderboard.forEach(function(entry, index) {
            const golferNames = entry.golfers.map(function(g) { return g.name; }).join(', ');
            html += '<tr><td><span class="rank rank-' + (index + 1) + '">#' + (index + 1) + '</span></td>';
            html += '<td>' + entry.player_name + '</td>';
            html += '<td><span class="golfer-names">' + golferNames + '</span></td>';
            html += '<td><span class="points">' + entry.total_points + '</span></td></tr>';
        });
        html += '</tbody></table>';

        container.innerHTML = html;
    }

    hideHistoryData() {
        document.getElementById('historyStatsSection')?.classList.add('hidden');
        document.getElementById('historyLeaderboardSection')?.classList.add('hidden');
        document.getElementById('historyEmptyState')?.classList.remove('hidden');
    }

    // Tournament Import methods
    async loadImportTournaments() {
        try {
            const response = await fetch(`${API_BASE}/tournaments`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('importTournamentSelect');
                if (select) {
                    select.innerHTML = '<option value="">Select a tournament</option>' +
                        tournaments.map(t => `
                            <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                        `).join('');
                }

                // Populate refresh tournament select (only ESPN-linked tournaments)
                const refreshSelect = document.getElementById('refreshTournamentSelect');
                const refreshBtn = document.getElementById('refreshScoresBtn');
                if (refreshSelect) {
                    const espnTournaments = tournaments.filter(t => t.espn_tournament_id);
                    if (espnTournaments.length > 0) {
                        refreshSelect.innerHTML = '<option value="">Select a tournament</option>' +
                            espnTournaments.map(t => `
                                <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                            `).join('');
                        refreshSelect.addEventListener('change', () => {
                            if (refreshBtn) refreshBtn.disabled = !refreshSelect.value;
                        });
                    } else {
                        refreshSelect.innerHTML = '<option value="">No ESPN-imported tournaments</option>';
                    }
                }
            }
        } catch (error) {
            console.error('Error loading tournaments for import:', error);
        }
    }

    async previewImportFile(event) {
        const file = event.target.files[0];
        const previewSection = document.getElementById('importPreviewSection');
        const resultSection = document.getElementById('importResultSection');

        resultSection.classList.add('hidden');

        if (!file) {
            previewSection.classList.add('hidden');
            this.pendingImportData = null;
            return;
        }

        const reader = new FileReader();
        reader.onload = async (e) => {
            try {
                const data = JSON.parse(e.target.result);

                if (!data.tournament || !data.players || !Array.isArray(data.players)) {
                    this.showToast('Invalid tournament.json format', 'error');
                    return;
                }

                this.pendingImportData = data;

                this.showLoading();
                try {
                    const response = await this.makeAdminRequest(
                        `${API_BASE}/admin/tournaments/import/preview`,
                        {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(data)
                        }
                    );

                    if (response.ok) {
                        const preview = await response.json();
                        this.importPreviewData = preview;
                        this.displayImportPreview(preview);
                        previewSection.classList.remove('hidden');
                    } else {
                        const error = await response.json();
                        this.showToast(error.message || 'Error previewing import', 'error');
                    }
                } catch (error) {
                    this.showToast('Error previewing import', 'error');
                    console.error(error);
                } finally {
                    this.hideLoading();
                }
            } catch (error) {
                this.showToast('Invalid JSON file', 'error');
                previewSection.classList.add('hidden');
            }
        };
        reader.readAsText(file);
    }

    async fetchEspnTournament() {
        const espnId = document.getElementById('espnTournamentId')?.value?.trim();
        if (!espnId) {
            this.showToast('Please enter an ESPN tournament ID', 'error');
            return;
        }

        const previewSection = document.getElementById('importPreviewSection');
        const resultSection = document.getElementById('importResultSection');
        resultSection.classList.add('hidden');

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/import/espn-preview`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ espn_tournament_id: espnId })
                }
            );

            if (response.ok) {
                const data = await response.json();
                this.importPreviewData = {
                    tournament_name: data.tournament_name,
                    matched: data.matched,
                    unmatched: data.unmatched,
                };
                this.pendingImportData = {
                    tournament: { name: data.tournament_name },
                    players: data.players,
                    espn_tournament_id: espnId,
                };
                this.displayImportPreview(this.importPreviewData);
                previewSection.classList.remove('hidden');
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error fetching ESPN data', 'error');
            }
        } catch (error) {
            this.showToast('Error fetching ESPN data', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayImportPreview(preview) {
        document.getElementById('importTournamentName').textContent = preview.tournament_name;
        document.getElementById('importMatchedCount').textContent = preview.matched.length;
        document.getElementById('importUnmatchedCount').textContent = preview.unmatched.length;

        const matchedContainer = document.getElementById('importMatchedList');
        if (preview.matched.length > 0) {
            let html = '<table><thead><tr><th>JSON Name</th><th>Matched To</th><th>Amateur</th><th>Rounds</th></tr></thead><tbody>';
            preview.matched.forEach(m => {
                html += `<tr>
                    <td>${m.json_name}</td>
                    <td>${m.golfer_name}</td>
                    <td>${m.is_amateur ? 'Yes' : 'No'}</td>
                    <td>${m.rounds_available.join(', ')}</td>
                </tr>`;
            });
            html += '</tbody></table>';
            matchedContainer.innerHTML = html;
        } else {
            matchedContainer.innerHTML = '<p class="loading">No automatic matches found.</p>';
        }

        const unmatchedContainer = document.getElementById('importUnmatchedList');
        if (preview.unmatched.length > 0) {
            let html = '';
            preview.unmatched.forEach((u, idx) => {
                html += `<div class="info-card" id="unmatchedCard_${idx}" style="margin-bottom: var(--spacing-sm); padding: var(--spacing-sm) var(--spacing-md);">
                    <div style="display: flex; align-items: center; gap: var(--spacing-md); flex-wrap: wrap;">
                        <span><strong>${u.json_name}</strong> (Rounds: ${u.rounds_available.join(', ')})</span>
                        <div style="display: flex; align-items: center; gap: var(--spacing-sm);">
                            <select class="form-input import-golfer-select" data-slug="${u.slug}" data-index="${idx}" style="max-width: 300px;">
                                <option value="">-- Skip this golfer --</option>
                                ${u.candidates.map(c => `<option value="${c.golfer_id}">${c.golfer_name}</option>`).join('')}
                            </select>
                            <input type="text" class="form-input import-golfer-search" data-index="${idx}" placeholder="Search golfers..." style="max-width: 200px;" oninput="app.searchImportGolfer(this, ${idx})">
                            <button type="button" class="btn btn-secondary btn-sm" onclick="app.toggleNewGolferForm(${idx})">Add as New Golfer</button>
                        </div>
                    </div>
                    <div id="importSearchResults_${idx}" class="hidden" style="margin-top: var(--spacing-sm);"></div>
                    <div id="newGolferForm_${idx}" class="hidden" style="margin-top: var(--spacing-sm); display: flex; align-items: center; gap: var(--spacing-sm); flex-wrap: wrap; padding: var(--spacing-sm); border: 1px solid var(--border); border-radius: var(--radius);">
                        <span style="font-weight: 500;">New: ${u.json_name}</span>
                        <label style="display: flex; align-items: center; gap: 4px;">
                            Group:
                            <select class="form-input new-golfer-group" data-index="${idx}" style="width: 60px;">
                                ${[1,2,3,4,5,6,7,8,9].map(g => `<option value="${g}">${g}</option>`).join('')}
                            </select>
                        </label>
                        <label style="display: flex; align-items: center; gap: 4px;">
                            <input type="checkbox" class="new-golfer-amateur" data-index="${idx}"> Amateur
                        </label>
                        <button type="button" class="btn btn-sm" style="background: var(--error); color: white;" onclick="app.cancelNewGolferForm(${idx})">Cancel</button>
                    </div>
                </div>`;
            });
            unmatchedContainer.innerHTML = html;

            document.querySelectorAll('.import-golfer-select').forEach(select => {
                select.addEventListener('change', () => this.updateImportCommitButton());
            });
        } else {
            unmatchedContainer.innerHTML = '<p class="loading">All golfers matched automatically!</p>';
        }

        document.getElementById('importUnmatchedSection').classList.toggle('hidden', preview.unmatched.length === 0);
        this.updateImportCommitButton();
    }

    async searchImportGolfer(input, index) {
        const query = input.value.trim();
        const resultsContainer = document.getElementById(`importSearchResults_${index}`);

        if (query.length < 2) {
            resultsContainer.classList.add('hidden');
            return;
        }

        try {
            const response = await fetch(`${API_BASE}/golfers`);
            if (response.ok) {
                const golfers = await response.json();
                const filtered = golfers.filter(g => g.name.toLowerCase().includes(query.toLowerCase())).slice(0, 10);

                if (filtered.length > 0) {
                    let html = '';
                    filtered.forEach(g => {
                        html += `<button type="button" class="btn btn-secondary btn-sm" style="margin: 2px;"
                            onclick="app.selectImportGolfer(${index}, '${g.id}', '${g.name.replace(/'/g, "\\'")}')">
                            ${g.name} (G${g.win_probability_group})
                        </button>`;
                    });
                    resultsContainer.innerHTML = html;
                    resultsContainer.classList.remove('hidden');
                } else {
                    resultsContainer.innerHTML = '<span style="color: var(--text-secondary);">No matches found</span>';
                    resultsContainer.classList.remove('hidden');
                }
            }
        } catch (error) {
            console.error('Error searching golfers:', error);
        }
    }

    selectImportGolfer(index, golferId, golferName) {
        const select = document.querySelector(`.import-golfer-select[data-index="${index}"]`);
        let optionExists = false;
        for (const opt of select.options) {
            if (opt.value === golferId) {
                opt.selected = true;
                optionExists = true;
                break;
            }
        }
        if (!optionExists) {
            const option = document.createElement('option');
            option.value = golferId;
            option.textContent = golferName;
            option.selected = true;
            select.appendChild(option);
        }

        const searchInput = document.querySelector(`.import-golfer-search[data-index="${index}"]`);
        if (searchInput) searchInput.value = '';
        document.getElementById(`importSearchResults_${index}`).classList.add('hidden');

        this.updateImportCommitButton();
    }

    toggleNewGolferForm(index) {
        const form = document.getElementById(`newGolferForm_${index}`);
        const select = document.querySelector(`.import-golfer-select[data-index="${index}"]`);
        if (form.classList.contains('hidden')) {
            form.classList.remove('hidden');
            form.style.display = 'flex';
            select.disabled = true;
            select.value = '';
            form.dataset.active = 'true';
        } else {
            this.cancelNewGolferForm(index);
        }
        this.updateImportCommitButton();
    }

    cancelNewGolferForm(index) {
        const form = document.getElementById(`newGolferForm_${index}`);
        const select = document.querySelector(`.import-golfer-select[data-index="${index}"]`);
        form.classList.add('hidden');
        form.dataset.active = '';
        select.disabled = false;
        this.updateImportCommitButton();
    }

    updateImportCommitButton() {
        const tournamentId = document.getElementById('importTournamentSelect')?.value;
        const hasData = !!this.pendingImportData;
        const btn = document.getElementById('importCommitBtn');
        if (btn) {
            btn.disabled = !tournamentId || !hasData;
        }
    }

    async refreshScores() {
        const tournamentId = document.getElementById('refreshTournamentSelect')?.value;
        if (!tournamentId) {
            this.showToast('Please select a tournament', 'error');
            return;
        }

        const resultSection = document.getElementById('refreshResultSection');
        const resultContent = document.getElementById('refreshResultContent');

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/${tournamentId}/scores/refresh`,
                { method: 'POST' }
            );

            const result = await response.json();

            if (response.ok) {
                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${result.total_scores_processed}</strong> hole scores processed.</p>`;
                html += `<p><strong>${result.golfers_updated}</strong> golfers updated, <strong>${result.golfers_skipped}</strong> skipped.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">`;
                    result.errors.forEach(err => { html += `<li>${err}</li>`; });
                    html += `</ul>`;
                }
                html += `</div>`;
                resultContent.innerHTML = html;
                resultSection.classList.remove('hidden');
                this.showToast('Scores refreshed successfully!', 'success');
            } else {
                this.showToast(result.message || 'Error refreshing scores', 'error');
                resultSection.classList.add('hidden');
            }
        } catch (error) {
            this.showToast('Error refreshing scores', 'error');
            console.error(error);
            resultSection.classList.add('hidden');
        } finally {
            this.hideLoading();
        }
    }

    async commitImport() {
        const tournamentId = document.getElementById('importTournamentSelect').value;
        if (!tournamentId) {
            this.showToast('Please select a target tournament', 'error');
            return;
        }

        if (!this.pendingImportData || !this.importPreviewData) {
            this.showToast('Please upload and preview a file first', 'error');
            return;
        }

        const preview = this.importPreviewData;
        const rawData = this.pendingImportData;

        const playerScores = [];

        for (const m of preview.matched) {
            const player = rawData.players.find(p => p.slug === m.slug);
            if (!player) continue;

            const entry = {
                golfer_id: m.golfer_id,
                rounds: player.rounds.map(r => ({
                    round_number: r.round_number,
                    holes: r.holes.map(h => ({
                        hole: h.hole,
                        strokes: h.score,
                        par: h.par
                    }))
                }))
            };
            if (player.espn_athlete_id) {
                entry.espn_athlete_id = player.espn_athlete_id;
            }
            playerScores.push(entry);
        }

        const newGolfers = [];
        const unmatchedSelects = document.querySelectorAll('.import-golfer-select');
        unmatchedSelects.forEach(select => {
            const idx = select.dataset.index;
            const slug = select.dataset.slug;
            const player = rawData.players.find(p => p.slug === slug);
            if (!player) return;

            const newGolferForm = document.getElementById(`newGolferForm_${idx}`);
            if (newGolferForm && newGolferForm.dataset.active === 'true') {
                const groupSelect = document.querySelector(`.new-golfer-group[data-index="${idx}"]`);
                const amateurCheckbox = document.querySelector(`.new-golfer-amateur[data-index="${idx}"]`);
                const unmatchedData = preview.unmatched[parseInt(idx)];
                newGolfers.push({
                    name: unmatchedData.json_name,
                    slug: slug,
                    win_probability_group: parseInt(groupSelect.value),
                    is_amateur: amateurCheckbox.checked,
                    espn_athlete_id: player.espn_athlete_id || null,
                    rounds: player.rounds.map(r => ({
                        round_number: r.round_number,
                        holes: r.holes.map(h => ({
                            hole: h.hole,
                            strokes: h.score,
                            par: h.par
                        }))
                    }))
                });
                return;
            }

            const golferId = select.value;
            if (!golferId) return;

            const entry = {
                golfer_id: golferId,
                rounds: player.rounds.map(r => ({
                    round_number: r.round_number,
                    holes: r.holes.map(h => ({
                        hole: h.hole,
                        strokes: h.score,
                        par: h.par
                    }))
                }))
            };
            if (player.espn_athlete_id) {
                entry.espn_athlete_id = player.espn_athlete_id;
            }
            playerScores.push(entry);
        });

        if (playerScores.length === 0 && newGolfers.length === 0) {
            this.showToast('No golfers selected for import', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/import/commit`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        tournament_id: tournamentId,
                        espn_tournament_id: rawData.espn_tournament_id || undefined,
                        player_scores: playerScores,
                        new_golfers: newGolfers
                    })
                }
            );

            const result = await response.json();
            const resultSection = document.getElementById('importResultSection');
            const resultContent = document.getElementById('importResultContent');

            if (response.ok) {
                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${result.total_scores_processed}</strong> hole scores imported for <strong>${playerScores.length}</strong> golfers.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">`;
                    result.errors.forEach(err => {
                        html += `<li>${err}</li>`;
                    });
                    html += `</ul>`;
                }
                html += `</div>`;
                resultContent.innerHTML = html;
                resultSection.classList.remove('hidden');
                this.showToast('Scores imported successfully!', 'success');
            } else {
                this.showToast(result.message || 'Error importing scores', 'error');
            }
        } catch (error) {
            this.showToast('Error importing scores', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }
}

// Initialize the app
const app = new FantasyGolfApp();
