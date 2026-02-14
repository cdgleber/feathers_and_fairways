// Fantasy Golf Application
const API_BASE = '/api';

class FantasyGolfApp {
    constructor() {
        this.currentSeason = null;
        this.selectedGolfers = new Map(); // Map<group, golferId>
        this.validatedKey = null;
        this.adminToken = localStorage.getItem('adminToken');
        this.selectedTournament = null;
        this.allSeasons = [];
        this.init();
    }

    async init() {
        this.initDarkMode();
        this.setupEventListeners();
        await this.loadInitialData();
    }

    initDarkMode() {
        // Load saved theme preference
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

        document.getElementById('createTeamForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.createTeam();
        });

        document.getElementById('createSeasonForm')?.addEventListener('submit', (e) => {
            e.preventDefault();
            this.createSeason();
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

        // Leaderboard tabs (scoped to avoid admin tab conflict)
        document.querySelectorAll('#leaderboardView .tab').forEach(tab => {
            tab.addEventListener('click', (e) => {
                const tabName = e.currentTarget.dataset.tab;
                this.switchTab(tabName);
            });
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

        // Tournament select
        document.getElementById('tournamentSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadTournamentLeaderboard(e.target.value);
            }
        });

        // History view selects
        document.getElementById('historySeasonSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadHistoryTournaments(e.target.value);
            }
        });
        document.getElementById('historyTournamentSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadHistoryTournamentData(e.target.value);
            } else {
                this.hideHistoryData();
            }
        });

        // Score file input
        document.getElementById('scoreFileInput')?.addEventListener('change', (e) => {
            this.previewScoreFile(e);
        });

        // Golfer file input
        document.getElementById('golferFileInput')?.addEventListener('change', (e) => {
            this.previewGolferFile(e);
        });

        // Group file input
        document.getElementById('groupFileInput')?.addEventListener('change', (e) => {
            this.previewGroupFile(e);
        });

        // Group upload tournament select
        document.getElementById('groupUploadTournament')?.addEventListener('change', () => {
            const btn = document.getElementById('uploadGroupsBtn');
            if (btn) {
                btn.disabled = !document.getElementById('groupUploadTournament').value || !this.pendingGroupData;
            }
        });

        // Score upload tournament select
        document.getElementById('scoreUploadTournament')?.addEventListener('change', () => {
            const btn = document.getElementById('uploadScoresBtn');
            if (btn) {
                btn.disabled = !document.getElementById('scoreUploadTournament').value || !this.pendingScoreData;
            }
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
            
            // Check admin auth for admin view
            if (viewName === 'admin') {
                this.checkAdminAuth();
            }
            
            // Load data for specific views
            if (viewName === 'leaderboard') {
                this.loadLeaderboards();
            } else if (viewName === 'history') {
                this.loadHistoryView();
            } else if (viewName === 'admin' && this.adminToken) {
                this.loadAdminData();
            }
        }
    }

    switchTab(tabName) {
        const view = document.getElementById('leaderboardView');
        view.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        view.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));

        view.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');
        document.getElementById(`${tabName}Tab`)?.classList.add('active');

        if (tabName === 'tournament') {
            this.loadTournaments();
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
        }
    }

    async loadInitialData() {
        this.showLoading();
        try {
            await Promise.all([
                this.loadActiveSeason(),
                this.loadStats(),
            ]);
        } catch (error) {
            this.showToast('Error loading data', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadActiveSeason() {
        try {
            const response = await fetch(`${API_BASE}/seasons/active`);
            if (response.ok) {
                this.currentSeason = await response.json();
                this.displaySeasonInfo();
            } else {
                document.getElementById('currentSeasonInfo').innerHTML = 
                    '<p class="text-center">No active season. Contact the commissioner to create one.</p>';
            }
        } catch (error) {
            console.error('Error loading active season:', error);
        }
    }

    displaySeasonInfo() {
        const container = document.getElementById('currentSeasonInfo');
        if (!this.currentSeason) return;

        container.innerHTML = `
            <h4 class="season-info-title">${this.currentSeason.name}</h4>
            <div class="season-info-grid">
                <div>
                    <p class="season-info-label">Year</p>
                    <p class="season-info-value">${this.currentSeason.year}</p>
                </div>
                <div>
                    <p class="season-info-label">Start Date</p>
                    <p class="season-info-value">${this.formatDate(this.currentSeason.start_date)}</p>
                </div>
                <div>
                    <p class="season-info-label">End Date</p>
                    <p class="season-info-value">${this.formatDate(this.currentSeason.end_date)}</p>
                </div>
            </div>
        `;
    }

    async loadStats() {
        try {
            const [golfersRes, teamsRes, tournamentsRes] = await Promise.all([
                fetch(`${API_BASE}/golfers`),
                this.currentSeason ? fetch(`${API_BASE}/teams/${this.currentSeason.id}`) : null,
                this.currentSeason ? fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`) : null,
            ]);

            if (golfersRes.ok) {
                const golfers = await golfersRes.json();
                document.getElementById('totalGolfers').textContent = golfers.length;
            }

            if (teamsRes?.ok) {
                const teams = await teamsRes.json();
                document.getElementById('totalTeams').textContent = teams.length;
            }

            if (tournamentsRes?.ok) {
                const tournaments = await tournamentsRes.json();
                const active = tournaments.filter(t => t.is_active).length;
                document.getElementById('activeTournaments').textContent = active;
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
                this.showToast('Access key validated successfully!', 'success');
                document.getElementById('teamBuilderSection').classList.remove('hidden');
                await this.loadTournamentsForSelection();
            } else if (result.already_used) {
                this.validatedKey = key;
                this.showToast('Access key already used - you can edit existing teams', 'info');
                document.getElementById('teamBuilderSection').classList.remove('hidden');
                await this.loadTournamentsForSelection();
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
            const teamsResponse = await fetch(`${API_BASE}/teams/${this.currentSeason.id}`);
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
                const result = await response.json();
                this.showToast(isUpdate ? 'Team updated successfully!' : 'Team created successfully!', 'success');
                
                // Hide form and show success message
                document.getElementById('teamBuilderSection').classList.add('hidden');
                const successSection = document.getElementById('teamCreatedSection');
                successSection.classList.remove('hidden');
                document.getElementById('teamCreatedMessage').textContent = 
                    isUpdate 
                        ? `Your team has been updated for the selected tournament!`
                        : `Welcome, ${playerName}! Your team has been created with your selected golfers.`;
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
        if (!this.currentSeason) return;

        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/leaderboard/${this.currentSeason.id}/detailed`);
            if (response.ok) {
                const leaderboard = await response.json();
                this.displaySeasonLeaderboard(leaderboard);
            }
        } catch (error) {
            this.showToast('Error loading leaderboard', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displaySeasonLeaderboard(leaderboard) {
        const container = document.getElementById('seasonLeaderboard');

        if (leaderboard.length === 0) {
            container.innerHTML = '<p class="loading">No teams yet. Be the first to join!</p>';
            return;
        }

        container.innerHTML = `
            <table>
                <thead>
                    <tr>
                        <th>Rank</th>
                        <th>Player</th>
                        <th>Golfers</th>
                        <th>Points</th>
                    </tr>
                </thead>
                <tbody>
                    ${leaderboard.map((entry, index) => `
                        <tr>
                            <td><span class="rank rank-${index + 1}">#${index + 1}</span></td>
                            <td>${entry.player_name}</td>
                            <td><span class="golfer-names">${entry.golfers.map(g => g.name).join(', ')}</span></td>
                            <td><span class="points">${entry.total_points}</span></td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    }

    async loadTournaments() {
        if (!this.currentSeason) return;

        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('tournamentSelect');
                
                select.innerHTML = '<option value="">Select a tournament</option>' +
                    tournaments.map(t => `
                        <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                    `).join('');
            }
        } catch (error) {
            console.error('Error loading tournaments:', error);
        }
    }

    async loadTournamentLeaderboard(tournamentId) {
        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/leaderboard/tournament/${tournamentId}`);
            if (response.ok) {
                const leaderboard = await response.json();
                this.displayTournamentLeaderboard(leaderboard);
            }
        } catch (error) {
            this.showToast('Error loading tournament leaderboard', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    displayTournamentLeaderboard(leaderboard) {
        const container = document.getElementById('tournamentLeaderboard');
        
        if (leaderboard.length === 0) {
            container.innerHTML = '<p class="loading">No scores recorded yet.</p>';
            return;
        }

        container.innerHTML = `
            <table>
                <thead>
                    <tr>
                        <th>Rank</th>
                        <th>Golfer</th>
                        <th>Points</th>
                    </tr>
                </thead>
                <tbody>
                    ${leaderboard.map((entry, index) => `
                        <tr>
                            <td><span class="rank rank-${index + 1}">#${index + 1}</span></td>
                            <td>${entry.golfer_name}</td>
                            <td><span class="points">${entry.total_points}</span></td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    }

    async loadAdminData() {
        await this.loadAdminStats();
        await this.loadTournaments();
        await this.loadTournamentsForScoreUpload();
        await this.loadTournamentsForGroupUpload();
    }

    async createSeason() {
        const name = document.getElementById('seasonName').value.trim();
        const year = parseInt(document.getElementById('seasonYear').value);
        const startDate = document.getElementById('seasonStart').value;
        const endDate = document.getElementById('seasonEnd').value;

        if (!name || !year || !startDate || !endDate) {
            this.showToast('Please fill in all fields', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/seasons`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, year, start_date: startDate, end_date: endDate })
            });

            if (response.ok) {
                const season = await response.json();
                this.showToast('Season created successfully!', 'success');
                this.currentSeason = season;
                document.getElementById('createSeasonForm').reset();
                await this.loadInitialData();
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error creating season', 'error');
            }
        } catch (error) {
            this.showToast('Error creating season', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async generateAccessKeys() {
        if (!this.currentSeason) {
            this.showToast('Please create a season first', 'error');
            return;
        }

        const count = parseInt(document.getElementById('keyCount').value);

        if (count < 1 || count > 50) {
            this.showToast('Please enter a number between 1 and 50', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(`${API_BASE}/admin/access-keys`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ season_id: this.currentSeason.id, count })
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

    async createTournament() {
        if (!this.currentSeason) {
            this.showToast('Please create a season first', 'error');
            return;
        }

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
                body: JSON.stringify({ 
                    season_id: this.currentSeason.id, 
                    name, 
                    start_date: startDate, 
                    end_date: endDate 
                })
            });

            if (response.ok) {
                this.showToast('Tournament created successfully!', 'success');
                document.getElementById('createTournamentForm').reset();
                await this.loadStats();
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

    // Score upload functions
    async loadTournamentsForScoreUpload() {
        if (!this.currentSeason) return;

        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('scoreUploadTournament');
                if (select) {
                    select.innerHTML = '<option value="">Select a tournament</option>' +
                        tournaments.map(t => `
                            <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                        `).join('');
                }
            }
        } catch (error) {
            console.error('Error loading tournaments for score upload:', error);
        }
    }

    async loadTournamentsForGroupUpload() {
        if (!this.currentSeason) return;

        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('groupUploadTournament');
                if (select) {
                    select.innerHTML = '<option value="">Select a tournament</option>' +
                        tournaments.map(t => `
                            <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                        `).join('');
                }
            }
        } catch (error) {
            console.error('Error loading tournaments for group upload:', error);
        }
    }

    previewScoreFile(event) {
        const file = event.target.files[0];
        const previewSection = document.getElementById('scorePreview');
        const previewContent = document.getElementById('scorePreviewContent');
        const uploadBtn = document.getElementById('uploadScoresBtn');

        if (!file) {
            previewSection.classList.add('hidden');
            uploadBtn.disabled = true;
            this.pendingScoreData = null;
            return;
        }

        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const data = JSON.parse(e.target.result);

                // Validate basic structure
                if (!data.pars || !Array.isArray(data.pars) || data.pars.length !== 18) {
                    this.showToast('Invalid file: pars must be an array of 18 values', 'error');
                    return;
                }

                if (!data.scores || !Array.isArray(data.scores) || data.scores.length === 0) {
                    this.showToast('Invalid file: scores array is missing or empty', 'error');
                    return;
                }

                this.pendingScoreData = data;

                // Show preview
                const golferCount = data.scores.length;
                const days = [...new Set(data.scores.map(s => s.day))].sort();
                const golferNames = data.scores.map(s => s.golfer);

                previewContent.innerHTML = `
                    <div class="info-card" style="margin-bottom: var(--spacing-md);">
                        <p><strong>${golferCount}</strong> golfer score entries</p>
                        <p>Days: <strong>${days.join(', ')}</strong></p>
                        <p>Golfers: <strong>${golferNames.join(', ')}</strong></p>
                        <p>Total hole scores: <strong>${golferCount * 18}</strong></p>
                    </div>
                `;

                previewSection.classList.remove('hidden');
                uploadBtn.disabled = !document.getElementById('scoreUploadTournament').value;
            } catch (error) {
                this.showToast('Invalid JSON file', 'error');
                previewSection.classList.add('hidden');
                uploadBtn.disabled = true;
            }
        };
        reader.readAsText(file);
    }

    async uploadScores() {
        const tournamentId = document.getElementById('scoreUploadTournament').value;

        if (!tournamentId) {
            this.showToast('Please select a tournament', 'error');
            return;
        }

        if (!this.pendingScoreData) {
            this.showToast('Please select a score file first', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/${tournamentId}/scores/upload`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(this.pendingScoreData)
                }
            );

            const result = await response.json();
            const resultSection = document.getElementById('scoreUploadResult');

            if (response.ok) {
                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${result.total_scores_processed}</strong> hole scores processed successfully.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">`;
                    result.errors.forEach(err => {
                        html += `<li>${err}</li>`;
                    });
                    html += `</ul>`;
                }
                html += `</div>`;
                resultSection.innerHTML = html;
                resultSection.classList.remove('hidden');
                this.showToast('Scores uploaded successfully!', 'success');
            } else {
                this.showToast(result.message || 'Error uploading scores', 'error');
            }
        } catch (error) {
            this.showToast('Error uploading scores', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    // Golfer upload functions
    previewGolferFile(event) {
        const file = event.target.files[0];
        const previewSection = document.getElementById('golferPreview');
        const previewContent = document.getElementById('golferPreviewContent');
        const uploadBtn = document.getElementById('uploadGolfersBtn');

        if (!file) {
            previewSection.classList.add('hidden');
            uploadBtn.disabled = true;
            this.pendingGolferData = null;
            return;
        }

        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const data = JSON.parse(e.target.result);

                if (!data.golfers || !Array.isArray(data.golfers) || data.golfers.length === 0) {
                    this.showToast('Invalid file: golfers array is missing or empty', 'error');
                    return;
                }

                this.pendingGolferData = data;

                const groupCounts = {};
                data.golfers.forEach(g => {
                    groupCounts[g.group] = (groupCounts[g.group] || 0) + 1;
                });

                const groupSummary = Object.entries(groupCounts)
                    .sort(([a], [b]) => a - b)
                    .map(([group, count]) => `Group ${group}: ${count}`)
                    .join(', ');

                previewContent.innerHTML = `
                    <div class="info-card" style="margin-bottom: var(--spacing-md);">
                        <p><strong>${data.golfers.length}</strong> golfers</p>
                        <p>${groupSummary}</p>
                        <p style="margin-top: 8px;"><strong>Names:</strong> ${data.golfers.map(g => g.name).join(', ')}</p>
                    </div>
                `;

                previewSection.classList.remove('hidden');
                uploadBtn.disabled = false;
            } catch (error) {
                this.showToast('Invalid JSON file', 'error');
                previewSection.classList.add('hidden');
                uploadBtn.disabled = true;
            }
        };
        reader.readAsText(file);
    }

    async uploadGolfers() {
        if (!this.pendingGolferData) {
            this.showToast('Please select a golfer file first', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/golfers/upload`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(this.pendingGolferData)
                }
            );

            const result = await response.json();
            const resultSection = document.getElementById('golferUploadResult');

            if (response.ok) {
                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${result.total_created}</strong> golfers created, <strong>${result.total_updated}</strong> updated.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">`;
                    result.errors.forEach(err => {
                        html += `<li>${err}</li>`;
                    });
                    html += `</ul>`;
                }
                html += `</div>`;
                resultSection.innerHTML = html;
                resultSection.classList.remove('hidden');
                this.showToast('Golfers uploaded successfully!', 'success');
                await this.loadStats();
            } else {
                this.showToast(result.message || 'Error uploading golfers', 'error');
            }
        } catch (error) {
            this.showToast('Error uploading golfers', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    // Tournament golfer group upload functions
    previewGroupFile(event) {
        const file = event.target.files[0];
        const previewSection = document.getElementById('groupPreview');
        const previewContent = document.getElementById('groupPreviewContent');
        const uploadBtn = document.getElementById('uploadGroupsBtn');

        if (!file) {
            previewSection.classList.add('hidden');
            uploadBtn.disabled = true;
            this.pendingGroupData = null;
            return;
        }

        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const data = JSON.parse(e.target.result);

                if (!data.groups || !Array.isArray(data.groups) || data.groups.length === 0) {
                    this.showToast('Invalid file: groups array is missing or empty', 'error');
                    return;
                }

                this.pendingGroupData = data;

                const groupCounts = {};
                data.groups.forEach(g => {
                    groupCounts[g.group] = (groupCounts[g.group] || 0) + 1;
                });

                const groupSummary = Object.entries(groupCounts)
                    .sort(([a], [b]) => a - b)
                    .map(([group, count]) => `Group ${group}: ${count}`)
                    .join(', ');

                previewContent.innerHTML = `
                    <div class="info-card" style="margin-bottom: var(--spacing-md);">
                        <p><strong>${data.groups.length}</strong> golfer group assignments</p>
                        <p>${groupSummary}</p>
                        <p style="margin-top: 8px;"><strong>Golfers:</strong> ${data.groups.map(g => `${g.golfer} (G${g.group})`).join(', ')}</p>
                    </div>
                `;

                previewSection.classList.remove('hidden');
                uploadBtn.disabled = !document.getElementById('groupUploadTournament').value;
            } catch (error) {
                this.showToast('Invalid JSON file', 'error');
                previewSection.classList.add('hidden');
                uploadBtn.disabled = true;
            }
        };
        reader.readAsText(file);
    }

    async uploadTournamentGroups() {
        const tournamentId = document.getElementById('groupUploadTournament').value;

        if (!tournamentId) {
            this.showToast('Please select a tournament', 'error');
            return;
        }

        if (!this.pendingGroupData) {
            this.showToast('Please select a groups file first', 'error');
            return;
        }

        this.showLoading();
        try {
            const response = await this.makeAdminRequest(
                `${API_BASE}/admin/tournaments/${tournamentId}/groups/upload`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(this.pendingGroupData)
                }
            );

            const result = await response.json();
            const resultSection = document.getElementById('groupUploadResult');

            if (response.ok) {
                let html = `<div class="info-card" style="border-left: 4px solid var(--success);">`;
                html += `<p><strong>${result.total_processed}</strong> golfer group assignments processed.</p>`;
                if (result.errors.length > 0) {
                    html += `<p style="color: var(--error); margin-top: 8px;"><strong>Errors:</strong></p>`;
                    html += `<ul style="margin-left: 16px;">`;
                    result.errors.forEach(err => {
                        html += `<li>${err}</li>`;
                    });
                    html += `</ul>`;
                }
                html += `</div>`;
                resultSection.innerHTML = html;
                resultSection.classList.remove('hidden');
                this.showToast('Tournament groups uploaded successfully!', 'success');
            } else {
                this.showToast(result.message || 'Error uploading groups', 'error');
            }
        } catch (error) {
            this.showToast('Error uploading tournament groups', 'error');
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

    async loadTournamentsForSelection() {
        if (!this.currentSeason) return;

        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
            if (response.ok) {
                const tournaments = await response.json();
                this.displayTournamentSelection(tournaments);
            }
        } catch (error) {
            console.error('Error loading tournaments:', error);
        }
    }

    displayTournamentSelection(tournaments) {
        const upcomingTournaments = tournaments.filter(t => {
            const startDate = new Date(t.start_date);
            const now = new Date();
            return startDate >= now;
        });

        if (upcomingTournaments.length === 0) {
            this.showToast('No upcoming tournaments available', 'error');
            return;
        }

        const container = document.getElementById('teamBuilderSection');
        container.innerHTML = `
            <h3 class="card-title">Select Tournament</h3>
            <div class="form-group">
                <label for="tournamentSelect" class="form-label">Choose a tournament to create/edit your team</label>
                <select id="tournamentSelectForTeam" class="form-input">
                    <option value="">-- Select Tournament --</option>
                    ${upcomingTournaments.map(t => `
                        <option value="${t.id}">${t.name} - ${this.formatDate(t.start_date)}</option>
                    `).join('')}
                </select>
            </div>
            <div id="teamFormContainer"></div>
        `;

        document.getElementById('tournamentSelectForTeam').addEventListener('change', (e) => {
            if (e.target.value) {
                this.selectedTournament = e.target.value;
                this.checkExistingTeam();
            }
        });
    }

    async checkExistingTeam() {
        if (!this.validatedKey || !this.selectedTournament) return;

        try {
            const response = await fetch(`${API_BASE}/teams/${this.currentSeason.id}`);
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
        if (!this.currentSeason) return;
        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
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

                // Find golfers that have scores
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

        // Find all days
        const days = [...new Set(scores.map(s => s.day))].sort((a, b) => a - b);

        let html = '<div class="score-editor-table"><table><thead><tr><th>Hole</th>';
        days.forEach(d => {
            html += `<th colspan="2">Round ${d}</th>`;
        });
        html += '</tr><tr><th></th>';
        days.forEach(() => {
            html += '<th>Strokes</th><th>To Par</th>';
        });
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

            // Find corresponding score_to_par input
            const parInput = document.querySelector(
                `.score-input[data-golfer-id="${golferId}"][data-day="${day}"][data-hole="${hole}"][data-field="score_to_par"]`
            );
            const scoreToPar = parseInt(parInput.value);

            scores.push({ golfer_id: golferId, day, hole, strokes, score_to_par: scoreToPar });
        });

        // Also check for changed score_to_par inputs without changed strokes
        const changedParInputs = document.querySelectorAll('#scoreEditorTable .score-input.changed[data-field="score_to_par"]');
        changedParInputs.forEach(input => {
            const golferId = input.dataset.golferId;
            const day = parseInt(input.dataset.day);
            const hole = parseInt(input.dataset.hole);

            // Skip if already added via strokes change
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
                // Refresh data
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
        if (!this.currentSeason) return;
        try {
            const response = await fetch(`${API_BASE}/tournaments/${this.currentSeason.id}`);
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

            // Group all golfers by group
            const golfersByGroup = {};
            allGolfers.forEach(g => {
                if (!golfersByGroup[g.win_probability_group]) {
                    golfersByGroup[g.win_probability_group] = [];
                }
                golfersByGroup[g.win_probability_group].push(g);
            });

            // Map current team golfers by group
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
                // Refresh the editor
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
        html += '<div class="admin-stat-item"><span class="admin-stat-value">' + stats.total_seasons + '</span><span class="admin-stat-label">Seasons</span></div>';
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

        if (stats.season_breakdown.length > 0) {
            html += '<h4 style="margin: var(--spacing-lg) 0 var(--spacing-md);">Season Breakdown</h4>';
            html += '<div class="leaderboard"><table><thead><tr><th>Season</th><th>Year</th><th>Tournaments</th><th>Teams</th><th>Scores</th></tr></thead><tbody>';
            stats.season_breakdown.forEach(function(s) {
                html += '<tr><td>' + s.season_name + '</td><td>' + s.season_year + '</td><td>' + s.tournament_count + '</td><td>' + s.team_count + '</td><td>' + s.score_count + '</td></tr>';
            });
            html += '</tbody></table></div>';
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
        await this.loadAllSeasons();

        if (this.allSeasons.length === 0) {
            this.hideHistoryData();
            document.getElementById('historyEmptyState').classList.remove('hidden');
            document.getElementById('historyEmptyMessage').textContent = 'No seasons available.';
            document.getElementById('historySeasonSelect').innerHTML = '<option value="">No seasons</option>';
            document.getElementById('historyTournamentSelect').innerHTML = '<option value="">No tournaments</option>';
            return;
        }

        const mostRecent = this.allSeasons[0];
        this.populateHistorySeasonSelect(mostRecent.id);
        await this.loadHistoryTournaments(mostRecent.id);
    }

    async loadAllSeasons() {
        try {
            const response = await fetch(`${API_BASE}/seasons`);
            if (response.ok) {
                this.allSeasons = await response.json();
            }
        } catch (error) {
            console.error('Error loading seasons:', error);
        }
    }

    populateHistorySeasonSelect(selectedId) {
        const select = document.getElementById('historySeasonSelect');
        select.innerHTML = this.allSeasons.map(s =>
            '<option value="' + s.id + '"' + (s.id === selectedId ? ' selected' : '') + '>' + s.name + ' (' + s.year + ')</option>'
        ).join('');
    }

    async loadHistoryTournaments(seasonId) {
        try {
            const response = await fetch(`${API_BASE}/tournaments/${seasonId}/completed`);
            if (response.ok) {
                const tournaments = await response.json();
                const select = document.getElementById('historyTournamentSelect');

                if (tournaments.length === 0) {
                    select.innerHTML = '<option value="">No completed tournaments</option>';
                    this.hideHistoryData();
                    document.getElementById('historyEmptyState').classList.remove('hidden');
                    document.getElementById('historyEmptyMessage').textContent = 'No completed tournaments for this season.';
                    return;
                }

                select.innerHTML = tournaments.map((t, i) =>
                    '<option value="' + t.id + '"' + (i === 0 ? ' selected' : '') + '>' + t.name + ' (' + this.formatDate(t.start_date) + ' - ' + this.formatDate(t.end_date) + ')</option>'
                ).join('');

                await this.loadHistoryTournamentData(tournaments[0].id);
            }
        } catch (error) {
            console.error('Error loading completed tournaments:', error);
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
}

// Initialize the app
const app = new FantasyGolfApp();