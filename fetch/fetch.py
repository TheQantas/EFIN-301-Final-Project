import requests
from bs4 import BeautifulSoup

TEAM_IDS = (
    ('ARI','CRD'),
    ('ATL','ATL'),
    ('BAL','RAV'),
    ('BUF','BUF'),
    ('CAR','CAR'),
    ('CHI','CHI'),
    ('CIN','CIN'),
    ('CLE','CLE'),
    ('DAL','DAL'),
    ('DEN','DEN'),
    ('DET','DET'),
    ('GB','GNB'),
    ('HOU','HTX'),
    ('IND','CLT'),
    ('JAX','JAX'),
    ('KC','KAN'),
    ('LV','RAI'),
    ('LAC','SDG'),
    ('LAR','RAM'),
    ('MIA','MIA'),
    ('MIN','MIN'),
    ('NE','NWE'),
    ('NO','NOR'),
    ('NYG','NYG'),
    ('NYJ','NYJ'),
    ('PHI','PHI'),
    ('PIT','PIT'),
    ('SF','SFO'),
    ('SEA','SEA'),
    ('TB','TAM'),
    ('TEN','OTI'),
    ('WAS','WAS'),
)

STAT_MAP = {
    'Fumble, Safety': 'Safety',
    'Blocked FG, Downs': 'Blocked FG',
    'Blocked Punt, Downs': 'Blocked Punt',
    'Blocked Punt, Safety': 'Safety',
}

def fetch(normal: str,alt: str,year: int) -> dict:
    print('FETCHING',normal,year)

    url = f"https://stathead.com/football/drive_finder.cgi?request=1&year_min={year}&year_max={year}&team_id={alt.lower()}&drive_num_gtlt=gt"
    response = requests.get(url)

    if response.status_code == 200:
        soup = BeautifulSoup(response.content,'html.parser')
        table = soup.find(id='drive_outcomes').find('tbody').find_all('tr')

        stat_dict = {
            'Team': normal,
            'Year': year,

            'Touchdown': 0,
            'Field Goal': 0,

            'Punt': 0,
            'Downs': 0,
            'Safety': 0,
            
            'Interception': 0,
            'Fumble': 0,

            'End of Half': 0,
            'End of Game': 0,

            'Missed FG': 0,
            'Blocked Punt': 0,
            'Blocked FG': 0,

            'All Turnovers': 0,
            'All Scores': 0
        }

        for row in table:
            cells = tuple(row.children)
            stat_name = cells[0].text
            stat_value = int(cells[1].text)
            
            if stat_name in STAT_MAP:
                stat_dict[STAT_MAP[stat_name]] += stat_value
            elif stat_name in stat_dict:
                stat_dict[stat_name] += stat_value
            else:
                print('MISSING',stat_name)

        return stat_dict

    else:
        raise Exception("Failed to retrieve the webpage. Status code:", response.status_code,alt,year)

def main():
    output = open('drives.csv','w')

    first = True
    for year in range(2002,2024):
        for (normal,alt) in TEAM_IDS:
            try:
                stat_dict = fetch(normal,alt,year)
            except:
                print('WTF',year,normal,alt)
                continue
            if first:
                output.write( ','.join( (element.lower().replace(' ','_') for element in stat_dict.keys()) ) )
                output.write('\n')
                first = False
            output.write( ','.join( (str(element) for element in stat_dict.values()) ) )
            output.write('\n')

main()