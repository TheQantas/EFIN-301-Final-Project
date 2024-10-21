import requests
from bs4 import BeautifulSoup
import time

# touchdowns
# field goals
# safety
# turnovers
# end of period

TEAMS_IDS = {
    'Arizona Cardinals': 'ARI',
    'Atlanta Falcons': 'ATL',
    'Baltimore Ravens': 'BAL',
    'Buffalo Bills': 'BUF',

    'Carolina Panthers': 'CAR',
    'Chicago Bears': 'CHI',
    'Cincinnati Bengals': 'CIN',
    'Cleveland Browns': 'CLE',

    'Dallas Cowboys': 'DAL',
    'Denver Broncos': 'DEN',
    'Detroit Lions': 'DET',
    'Green Bay Packers': 'GB',

    'Houston Texans': 'HOU',
    'Indianapolis Colts': 'IND',
    'Jacksonville Jaguars': 'JAX',
    'Kansas City Chiefs': 'KC',

    'Las Vegas Raiders': 'LV',
    'Oakland Raiders': 'LV',
    'Los Angeles Chargers': 'LAC',
    'San Diego Chargers': 'LAC',
    'Los Angeles Rams': 'LAR',
    'St. Louis Rams': 'LAR',
    'Miami Dolphins': 'MIA',

    'Minnesota Vikings': 'MIN',
    'New England Patriots': 'NE',
    'New Orleans Saints': 'NO',
    'New York Giants': 'NYG',

    'New York Jets': 'NYJ',
    'Philadelphia Eagles': 'PHI',
    'Pittsburgh Steelers': 'PIT',
    'San Francisco 49ers': 'SF',

    'Seattle Seahawks': 'SEA',
    'Tampa Bay Buccaneers': 'TB',
    'Tennessee Titans': 'TEN',
    'Washington Commanders': 'WAS',
    'Washington Football Team': 'WAS',
    'Washington Redskins': 'WAS',
}

DATA_SHAPE = {
    'touchdown': 0,
    'field_goal': 0,
    'missed_fg': 0,
    'punt': 0,
    'turnovers': 0,
    'safety': 0,
    'total': 0,
    'end_of_period': 0,
    'blocked_punt': 0,
    'downs': 0,
    'blocked_fg': 0,
    'playoff': 0
}

PLAYOFFS = (
    ('LV','TEN','PIT','NYJ','IND','CLE' , 'PHI','TB','GB','SF','NYG','ATL'), #2002
    ('NE','KC','IND','BAL','TEN','DEN' , 'PHI','LAR','CAR','GB','SEA','DAL'), #2003
    ('PIT','NE','IND','LAC','NYJ','DEN' , 'PHI','ATL','GB','SEA','LAR','MIN'), #2004
    ('IND','DEN','CIN','NE','JAX','PIT' , 'SEA','CHI','TB','NYJ','CAR','WAS'), #2005
    ('LAC','BAL','IND','NE','NYJ','KC' , 'CHI','NO','PHI','SEA','DAL','NYG'), #2006
    ('NE','IND','LAC','PIT','JAX','TEN' , 'DAL','GB','SEA','TB','NYG','WAS'), #2007
    ('TEN','PIT','MIA','LAC','IND','BAL' , 'NYG','CAR','MIN','ARI','ATL','PHI'), #2008
    ('IND','LAC','NE','CIN','NYJ','BAL' , 'NO','MIN','DAL','ARI','GB','PHI'), #2009
    ('NE','PIT','IND','KC','BAL','NYJ' , 'ATL','CHI','PHI','SEA','NO','GB'), #2010
    ('NE','BAL','HOU','DEN','PIT','CIN' , 'GB','SF','NO','NYG','ATL','DET'), #2011
    ('DEN','NE','HOU','BAL','IND','CIN' , 'ATL','SF','GB','WAS','SEA','MIN'), #2012
    ('DEN','NE','CIN','IND','KC','LAC' , 'SEA','CAR','PHI','GB','SF','NO'), #2013
    ('NE','DEN','PIT','IND','CIN','BAL' , 'SEA','GB','DAL','CAR','ARI','DET'), #2014
    ('DEN','NE','CIN','HOU','KC','PIT' , 'CAR','ARI','MIN','WAS','GB','SEA'), #2015
    ('NE','KC','PIT','HOU','LV','MIA' , 'DAL','ATL','SEA','GB','NYG','DET'), #2016
    ('NE','PIT','JAX','KC','TEN','BUF' , 'PHI','MIN','LAR','NO','CAR','ATL'), #2017
    ('KC','NE','HOU','BAL','LAC','IND' , 'NO','LAR','CHI','DAL','SEA','PHI'), #2018
    ('BAL','KC','NE','HOU','BUF','TEN' , 'SF','GB','NO','PHI','SEA','MIN'), #2019
    ('KC','BUF','PIT','TEN','BAL','CLE','IND' , 'GB','NO','SEA','WAS','TB','LAR','CHI'), #2020
    ('TEN','KC','BUF','CIN','LV','NE','PIT' , 'GB','TB','DAL','LAR','ARI','SF','PHI'), #2021
    ('KC','BUF','CIN','JAX','LAC','BAL','MIA' , 'PHI','SF','MIN','TB','DAL','NYG','SEA'), #2022
    ('BAL','BUF','KC','HOU','CLE','MIA','PIT' , 'SF','DAL','DET','TB','PHI','LAR','GB'), #2023
)

def get_defense(year: int) -> dict[str,dict]:
    url = f"https://www.pro-football-reference.com/years/{year}/opp.htm"
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(response.status_code,year)
    print('defense year',year,'code',response.status_code)
    
    team_dict = {}
    for team_id in TEAMS_IDS.values():
        team_dict[team_id] = DATA_SHAPE.copy()
        team_dict[team_id]['playoff'] = 1 if team_id in PLAYOFFS[year-2002] else 0
    
    clean_content = response.content.decode('utf-8').replace('\n','').replace('\t','').replace('<!--<div class="table_container"','<div class="table_container"').replace('</tfoot></table></div>-->','</tfoot></table></div>')
    open('wtf2.html','w').write(clean_content)
    soup = BeautifulSoup(clean_content,'html.parser')


    for row in soup.find(id='team_stats').find('tbody').find_all('tr'): #tds and turnovers
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        team_dict[team_id]['touchdown'] = int( cells[13].text ) + int( cells[19].text )
        team_dict[team_id]['turnovers'] = int( cells[7].text )

    for row in soup.find(id='kicking').find('tbody').find_all('tr'): #kicking
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        made = int( cells[4].text )
        attempted = int( cells[3].text )
        team_dict[team_id]['field_goal'] = made
        team_dict[team_id]['missed_fg'] = attempted - made

    for row in soup.find(id='punting').find('tbody').find_all('tr'): #punting
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        team_dict[team_id]['punt'] = int( cells[3].text )
        blocked = cells[6].text
        team_dict[team_id]['blocked_punt'] = 0 if blocked == '' else int(blocked)   

    for row in soup.find(id='team_scoring').find('tbody').find_all('tr'): #safety
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        safety = cells[18].text
        team_dict[team_id]['safety'] = 0 if safety == '' else int(safety)


    for row in soup.find(id='drives').find('tbody').find_all('tr'): #overall
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        dict = team_dict[team_id]
        dict['total'] = int( cells[3].text )
        # print(team_id,int(cells[3].text))
        dict['end_of_period'] = dict['total'] - dict['touchdown'] - dict['field_goal'] - dict['missed_fg'] - dict['punt'] - dict['turnovers'] - dict['safety'] - dict['blocked_punt']
        # if team_id == 'CAR':
            # print('A!',cells[3].text)
            # print(dict)

    # print('final',team_dict['CAR'])

    return team_dict
        
def main():
    file = open('data/drives_defense.csv','w')
    file.write('team,year,touchdown,field_goal,missed_fg,punt,turnovers,safety,total,end_of_period,blocked_punt,downs,blocked_fg,playoff\n')

    for year in range(2002,2024):
        time.sleep(1)
        team_dict = get_defense(year)
        seen = set()
        for team_id in TEAMS_IDS.values():
            if team_id in seen:
                continue
            seen.add(team_id)
            # if team_id == 'CAR':
                # print(team_dict[team_id])
            file.write(f'{team_id},{year},{','.join( [str(s) for s in team_dict[team_id].values()] )}\n')
        # break
    
if __name__ == "__main__":
    main()