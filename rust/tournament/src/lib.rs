use std::collections::HashMap;

struct Tournament {
    teams: HashMap<String, Team>,
}

struct Team {
    name: String,
    matches_played: u16,
    wins: u16,
    draws: u16,
    losses: u16,
    score: u16,
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    let mut result_string = "Team                           | MP |  W |  D |  L |  P\n".to_string();

    if match_results.is_empty() {
        result_string.pop();
        return result_string;
    }

    for game in match_results.split('\n') {
        let mut result = game.split(';');
        tournament.match_result(
            result.next().unwrap(),
            result.next().unwrap(),
            result.next().unwrap(),
        );
    }

    let mut sort_tournament = tournament.teams.values().collect::<Vec<&Team>>();
    sort_tournament.sort_by_key(|team| team.name.clone());
    sort_tournament.reverse();
    sort_tournament.sort_by_key(|team| team.score);
    sort_tournament.reverse();

    for team in sort_tournament.iter() {
        result_string += format!(
            "{:<31}|  {} |  {} |  {} |  {} |  {}\n",
            team.name, team.matches_played, team.wins, team.draws, team.losses, team.score
        )
        .as_str();
    }

    result_string.pop();
    result_string
}

impl Tournament {
    fn new() -> Self {
        Tournament {
            teams: HashMap::new(),
        }
    }

    fn get_team(&mut self, name: &str) -> &mut Team {
        if !self.teams.contains_key(name) {
            self.teams.insert(name.to_string(), Team::new(name));
        }
        self.teams.get_mut(name).unwrap()
    }

    fn match_result(&mut self, team1: &str, team2: &str, result: &str) {
        let team1 = self.get_team(team1);
        match result {
            "win" => team1.win(),
            "draw" => team1.draw(),
            "loss" => team1.loss(),
            _ => {}
        }

        let team2 = self.get_team(team2);
        match result {
            "win" => team2.loss(),
            "draw" => team2.draw(),
            "loss" => team2.win(),
            _ => {}
        }
    }
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            score: 0,
        }
    }

    fn win(&mut self) {
        self.matches_played += 1;
        self.wins += 1;
        self.score += 3;
    }

    fn draw(&mut self) {
        self.matches_played += 1;
        self.draws += 1;
        self.score += 1;
    }

    fn loss(&mut self) {
        self.matches_played += 1;
        self.losses += 1;
    }
}
