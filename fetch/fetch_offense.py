import requests
from bs4 import BeautifulSoup
import time

from fetch_defense import TEAMS_IDS,DATA_SHAPE,PLAYOFFS

# touchdowns
# field goals
# safety
# turnovers
# end of period

def get_offense(year: int) -> dict[str,dict]:
    url = f"https://www.pro-football-reference.com/years/{year}/#all_team_stats"
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(response.status_code,year)
    print('offense year',year,'code',response.status_code)
    
    team_dict = {}
    for team_id in TEAMS_IDS.values():
        team_dict[team_id] = DATA_SHAPE.copy()
        team_dict[team_id]['playoff'] = 1 if team_id in PLAYOFFS[year-2002] else 0
    
    clean_content = response.content.decode('utf-8').replace('\n','').replace('\t','').replace('<!--<div class="table_container"','<div class="table_container"').replace('</tfoot></table></div>-->','</tfoot></table></div>')
    # open('wtf2.html','w').write(clean_content)
    soup = BeautifulSoup(clean_content,'html.parser')


    for row in soup.find(id='team_stats').find('tbody').find_all('tr'): #tds and turnovers
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        team_dict[team_id]['touchdown'] = int( cells[13].text ) + int( cells[19].text )
        team_dict[team_id]['turnovers'] = int( cells[7].text )

    for row in soup.find(id='kicking').find('tbody').find_all('tr'): #kicking
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        made = int( cells[14].text )
        attempted = int( cells[13].text )
        team_dict[team_id]['field_goal'] = made
        team_dict[team_id]['missed_fg'] = attempted - made

    for row in soup.find(id='punting').find('tbody').find_all('tr'): #punting
        cells = tuple(row.children)
        team_id = TEAMS_IDS[ cells[1].text ]
        team_dict[team_id]['punt'] = int( cells[3].text )
        blocked = cells[14].text
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
        dict['end_of_period'] = dict['total'] - dict['touchdown'] - dict['field_goal'] - dict['missed_fg'] - dict['punt'] - dict['turnovers'] - dict['safety'] - dict['blocked_punt']

    return team_dict
        
def main():
    file = open('data/drives_offense.csv','w')
    file.write('team,year,touchdown,field_goal,missed_fg,punt,turnovers,safety,total,end_of_period,blocked_punt,downs,blocked_fg,playoff\n')

    for year in range(2002,2024):
        time.sleep(1)
        team_dict = get_offense(year)
        seen = set()
        for team_id in TEAMS_IDS.values():
            if team_id in seen:
                continue
            seen.add(team_id)
            file.write(f'{team_id},{year},{','.join( [str(s) for s in team_dict[team_id].values()] )}\n')
    
if __name__ == "__main__":
    main()