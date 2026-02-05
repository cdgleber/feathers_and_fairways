// Fantasy Golf Application
const API_BASE = '/api';

class FantasyGolfApp {
    constructor() {
        this.currentSeason = null;
        this.selectedGolfers = new Map(); // Map<group, golferId>
        this.validatedKey = null;
        this.init();
    }

    async init() {
        this.setupEventListeners();
        await this.loadInitialData();
    }

    setupEventListeners() {
        // Navigation
        document.querySelectorAll('.nav-btn, .mobile-nav-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const view = e.currentTarget.dataset.view;
                this.showView(view);
                this.closeMobileNav();
            });
        });

        // Mobile menu toggle
        const mobileMenuBtn = document.getElementById('mobileMenuBtn');
        const mobileNav = document.getElementById('mobileNav');
        mobileMenuBtn?.addEventListener('click', () => {
            mobileNav.classList.toggle('active');
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

        // Tabs
        document.querySelectorAll('.tab').forEach(tab => {
            tab.addEventListener('click', (e) => {
                const tabName = e.currentTarget.dataset.tab;
                this.switchTab(tabName);
            });
        });

        // Tournament select
        document.getElementById('tournamentSelect')?.addEventListener('change', (e) => {
            if (e.target.value) {
                this.loadTournamentLeaderboard(e.target.value);
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
            
            // Load data for specific views
            if (viewName === 'leaderboard') {
                this.loadLeaderboards();
            } else if (viewName === 'admin') {
                this.loadAdminData();
            }
        }
    }

    switchTab(tabName) {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
        
        document.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');
        document.getElementById(`${tabName}Tab`)?.classList.add('active');

        if (tabName === 'tournament') {
            this.loadTournaments();
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
            <h4 style="font-size: 20px; font-weight: 600; margin-bottom: 12px;">
                ${this.currentSeason.name}
            </h4>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;">
                <div>
                    <p style="color: var(--text-secondary); font-size: 14px;">Year</p>
                    <p style="font-weight: 600;">${this.currentSeason.year}</p>
                </div>
                <div>
                    <p style="color: var(--text-secondary); font-size: 14px;">Start Date</p>
                    <p style="font-weight: 600;">${this.formatDate(this.currentSeason.start_date)}</p>
                </div>
                <div>
                    <p style="color: var(--text-secondary); font-size: 14px;">End Date</p>
                    <p style="font-weight: 600;">${this.formatDate(this.currentSeason.end_date)}</p>
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
                await this.loadGolfersForSelection();
            } else if (result.already_used) {
                this.showToast('This access key has already been used', 'error');
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
            const response = await fetch(`${API_BASE}/golfers`);
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

            for (let i = 1; i <= 6; i++) {
                const groupGolfers = groups[i] || [];
                const groupDiv = document.createElement('div');
                groupDiv.className = 'golfer-group';
                
                groupDiv.innerHTML = `
                    <div class="golfer-group-header">
                        <span class="material-icons">sports_golf</span>
                        Group ${i} ${i === 1 ? '(Highest Probability)' : i === 6 ? '(Lowest Probability)' : ''}
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
                                <label for="golfer-${golfer.id}">${golfer.name}</label>
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
        for (let i = 1; i <= 6; i++) {
            const selected = this.selectedGolfers.get(i);
            const badge = document.createElement('div');
            badge.className = `selection-badge ${selected ? 'selected' : ''}`;
            badge.innerHTML = `
                <span>Group ${i}:</span>
                <strong>${selected ? selected.name : 'Not selected'}</strong>
            `;
            container.appendChild(badge);
        }

        createBtn.disabled = this.selectedGolfers.size !== 6;
    }

    async createTeam() {
        const playerName = document.getElementById('playerName').value.trim();
        
        if (!playerName) {
            this.showToast('Please enter your name', 'error');
            return;
        }

        if (this.selectedGolfers.size !== 6) {
            this.showToast('Please select one golfer from each group', 'error');
            return;
        }

        const golferIds = Array.from(this.selectedGolfers.values()).map(g => g.id);

        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/teams`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    key_code: this.validatedKey,
                    player_name: playerName,
                    golfer_ids: golferIds
                })
            });

            if (response.ok) {
                const result = await response.json();
                this.showToast('Team created successfully!', 'success');
                
                // Hide form and show success message
                document.getElementById('teamBuilderSection').classList.add('hidden');
                const successSection = document.getElementById('teamCreatedSection');
                successSection.classList.remove('hidden');
                document.getElementById('teamCreatedMessage').textContent = 
                    `Welcome, ${playerName}! Your team has been created with your selected golfers.`;
            } else {
                const error = await response.json();
                this.showToast(error.message || 'Error creating team', 'error');
            }
        } catch (error) {
            this.showToast('Error creating team', 'error');
            console.error(error);
        } finally {
            this.hideLoading();
        }
    }

    async loadLeaderboards() {
        if (!this.currentSeason) return;

        this.showLoading();
        try {
            const response = await fetch(`${API_BASE}/leaderboard/${this.currentSeason.id}`);
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
                        <th>Points</th>
                    </tr>
                </thead>
                <tbody>
                    ${leaderboard.map((entry, index) => `
                        <tr>
                            <td><span class="rank rank-${index + 1}">#${index + 1}</span></td>
                            <td>${entry.player_name}</td>
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
        // Load any admin-specific data here
        await this.loadTournaments();
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
            const response = await fetch(`${API_BASE}/seasons`, {
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
            const response = await fetch(`${API_BASE}/access-keys`, {
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
                <button class="btn btn-secondary" style="padding: 4px 12px; font-size: 14px;" 
                        onclick="app.copyToClipboard('${key.key_code}')">
                    <span class="material-icons" style="font-size: 16px;">content_copy</span>
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
            const response = await fetch(`${API_BASE}/golfers`, {
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
            const response = await fetch(`${API_BASE}/tournaments`, {
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
}

// Initialize the app
const app = new FantasyGolfApp();