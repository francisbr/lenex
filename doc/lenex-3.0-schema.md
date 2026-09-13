# Lenex 3.0 — AI Schema Reference

Schema only, no prose. Tables ordered top-down following the Lenex tree (parent before child, each type defined once at first use). `Type` cells link to the table defining that type. `Req`: **req** = exactly one/must be present, **opt** = zero or one, **req[]** = array, must be present (may be empty), **opt[]** = array, optional.

## Primitive types

| Type | Format |
|---|---|
| string | Any character; `< > " ' &` must be escaped as `&lt; &gt; &quot; &apos; &amp;` |
| string-intl | Same as string, ASCII #32–#127 only |
| number | Signed 32-bit integer |
| enum | One of a fixed set of values (listed per field) |
| date | `YYYY-MM-DD` |
| daytime | `HH:MM`, hours 0–24, minutes 0–59 |
| currency | Integer, cents (100 = 1.00) |
| swimtime | `HH:MM:SS.ss` or `NT` (no time) |
| reactiontime | Integer hundredths of a second, signed (`+14`, `-14`); `0` if zero; empty if unknown |
| unique-id | `[A-Z]` + number, optional separators (space/dash/point), ignored in comparison; globally unique |

## Tree

```
LENEX
 CONSTRUCTOR
 MEETS[]: Meet
  MEET
   SESSIONS[]: Session
    SESSION
     POOL, EVENTS[]: Event
      EVENT
       AGEGROUPS[]: Agegroup
        AGEGROUP → RANKINGS[]: Ranking
       HEATS[]: Heat
       SWIMSTYLE
       TIMESTANDARDREFS[]: TimeStandardRef
     JUDGES[]: Judge
   CLUBS[]: Club
    CLUB
     ATHLETES[]: Athlete
      ATHLETE → ENTRIES[]: Entry, RESULTS[]: Result
     RELAYS[]: Relay
      RELAY → ENTRIES[]: Entry, RESULTS[]: Result
     OFFICIALS[]: Official
 RECORDLISTS[]: RecordList
  RECORDLIST → RECORDS[]: Record
 TIMESTANDARDLISTS[]: TimeStandardList
  TIMESTANDARDLIST → TIMESTANDARDS[]: TimeStandard
```

## LENEX

| Field | Type | Req | Notes |
|---|---|---|---|
| version | string | req | Lenex format version |
| CONSTRUCTOR | [Constructor](#constructor) | req | |
| MEETS | [Meet](#meet)[] | opt[] | |
| RECORDLISTS | [RecordList](#recordlist)[] | opt[] | |
| TIMESTANDARDLISTS | [TimeStandardList](#timestandardlist)[] | opt[] | |

## Constructor

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | req | |
| version | string | req | |
| registration | string | opt | |
| CONTACT | [Contact](#contact) | req | |

## Contact

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | opt | not used under Official |
| email | string | req(Constructor)/opt | required only under Constructor |
| city | string | opt | |
| country | enum\<CountryCode\> | opt | see [Enum sources](#enum-code-sources) |
| state | string | opt | |
| street | string | opt | |
| street2 | string | opt | |
| zip | string | opt | |
| phone | string | opt | |
| mobile | string | opt | |
| fax | string | opt | |
| internet | string | opt | full URL incl. `http://` |

## Meet

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | req | |
| name.en | string-intl | opt | |
| city | string | req | |
| city.en | string-intl | opt | |
| nation | enum\<NationCode\> | req | |
| state | string | opt | |
| course | enum\<CourseCode\> | opt | if absent, every Session needs `course` |
| altitude | number | opt | |
| number | string | opt | sanction number |
| type | string | opt | default = FINA rules |
| timing | enum\<AUTOMATIC\|MANUAL3\|MANUAL1\> | opt | |
| maxentries | number | opt | per athlete/relay; per-event cap = Event.maxentries |
| deadline | date | opt | |
| deadlinetime | daytime | opt | |
| entrystartdate | date | opt | |
| entrytype | enum\<OPEN\|INVITATION\> | opt | |
| hostclub | string | opt | |
| hostclub.url | string | opt | |
| organizer | string | opt | |
| organizer.url | string | opt | |
| result.url | string | opt | deep link to results |
| swrid | unique-id | opt | swimrankings.net meet id |
| AGEDATE | [AgeDate](#agedate) | opt | default `{type: YEAR, value: first session date}` |
| CONTACT | [Contact](#contact) | opt | |
| FEES | [Fee](#fee)[] | opt[] | global club/athlete/relay/team fees |
| POINTTABLE | [PointTable](#pointtable) | opt | |
| POOL | [Pool](#pool) | opt | |
| QUALIFY | [Qualify](#qualify) | opt | |
| SESSIONS | [Session](#session)[] | req[] | |
| CLUBS | [Club](#club)[] | opt[] | |

## AgeDate

| Field | Type | Req | Notes |
|---|---|---|---|
| type | enum\<YEAR\|DATE\|POR\|CAN.FNQ\|LUX\> | req | age-calculation method |
| value | date | req | |

## Fee

| Field | Type | Req | Notes |
|---|---|---|---|
| type | enum\<CLUB\|ATHLETE\|RELAY\|TEAM\|LATEENTRY.INDIVIDUAL\|LATEENTRY.RELAY\> | req(collection)/opt | `LATEENTRY.*` only under Meet.FEES; required only inside a FEES collection |
| value | currency | req | |
| currency | enum\<CurrencyCode\> | opt | |

## PointTable

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | req | |
| version | string | req | |
| pointtableid | number | opt | see [Enum sources](#enum-code-sources) |

## Pool

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | opt | |
| type | enum\<INDOOR\|OUTDOOR\|LAKE\|OCEAN\> | opt | |
| lanemin | number | opt | |
| lanemax | number | opt | lane count = lanemax − lanemin + 1 |
| temperature | number | opt | |

## Qualify

| Field | Type | Req | Notes |
|---|---|---|---|
| from | date | req | |
| until | date | opt | default = day before meet start |
| conversion | enum\<NONE\|FINA_POINTS\|PERCENT_LINEAR\|NON_CONFORMING_LAST\> | opt | default NONE |
| percent | number | opt | used with conversion=PERCENT_LINEAR |

## Session

| Field | Type | Req | Notes |
|---|---|---|---|
| number | number | req | unique per meet |
| date | date | req | |
| daytime | daytime | opt | |
| name | string | opt | |
| course | enum\<CourseCode\> | opt | overrides Meet.course |
| warmupfrom | daytime | opt | |
| warmupuntil | daytime | opt | |
| teamleadermeeting | daytime | opt | |
| officialmeeting | daytime | opt | |
| POOL | [Pool](#pool) | opt | if different per session |
| FEES | [Fee](#fee)[] | opt[] | |
| EVENTS | [Event](#event)[] | req[] | |
| JUDGES | [Judge](#judge)[] | opt[] | |

## Event

| Field | Type | Req | Notes |
|---|---|---|---|
| eventid | number | req | unique per meet |
| number | number | req | unique per meet; rounds of same event may share |
| daytime | daytime | opt | |
| order | number | opt | |
| gender | enum\<A\|M\|F\|X\> | opt | default A |
| round | enum\<TIM\|FHT\|FIN\|SEM\|QUA\|PRE\|SOP\|SOS\|SOQ\|GER.RES\> | opt | default TIM |
| run | number | opt | default 1 |
| preveventid | number | opt | default -1 (no previous event) |
| type | enum\<""\|MASTERS\> | opt | |
| timing | enum\<AUTOMATIC\|MANUAL3\|MANUAL1\> | opt | falls back to Meet.timing |
| maxentries | number | opt | per club |
| SWIMSTYLE | [SwimStyle](#swimstyle) | req | |
| AGEGROUPS | [Agegroup](#agegroup)[] | opt[] | forbidden if round=FHT |
| HEATS | [Heat](#heat)[] | opt[] | |
| FEE | [Fee](#fee) | opt | |
| TIMESTANDARDREFS | [TimeStandardRef](#timestandardref)[] | opt[] | |

## Agegroup

| Field | Type | Req | Notes |
|---|---|---|---|
| agegroupid | number | req | required+unique within Agegroups when child of Event |
| agemin | number | req | -1=no lower bound |
| agemax | number | req | -1=no upper bound |
| gender | enum\<M\|F\|X\> | opt | not allowed under RecordList/TimeStandardList |
| calculate | enum\<SINGLE\|TOTAL\> | opt | default SINGLE (relay age calc) |
| levelmin | string | opt | A-Z |
| levelmax | string | opt | A-Z |
| levels | string | opt | comma-separated codes |
| handicap | enum\<1-15\|20\|34\|49\> | opt | |
| name | string | opt | |
| RANKINGS | [Ranking](#ranking)[] | opt[] | |

## Ranking

| Field | Type | Req | Notes |
|---|---|---|---|
| place | number | req | |
| resultid | number | req | → [Result](#result).resultid |
| order | number | opt | fallback sort key = place |

## Heat

| Field | Type | Req | Notes |
|---|---|---|---|
| heatid | number | req | unique per meet; required if referenced by Entry/Result |
| number | number | req | unique per event, incl. across finals of different agegroups |
| daytime | daytime | opt | |
| order | number | opt | |
| final | enum\<A\|B\|C\|D\> | opt | |
| status | enum\<SEEDED\|INOFFICIAL\|OFFICIAL\> | opt | |
| agegroupid | number | opt | → [Agegroup](#agegroup).agegroupid |

## SwimStyle

| Field | Type | Req | Notes |
|---|---|---|---|
| distance | number | req | per-athlete distance for relays |
| relaycount | number | req | 1=individual, >1=relay |
| stroke | enum\<APNEA\|BACK\|BREAST\|FLY\|FREE\|IMMERSION\|MEDLEY\|SURFACE\|UNKNOWN\> | req | |
| technique | enum\<DIVE\|GLIDE\|KICK\|PULL\|START\|TURN\> | opt | empty=normal swimming |
| code | string | opt | ≤6 chars, if stroke=UNKNOWN |
| name | string | opt | if stroke=UNKNOWN |
| swimstyleid | number | opt | unique id when stroke=UNKNOWN |

## TimeStandardRef

| Field | Type | Req | Notes |
|---|---|---|---|
| timestandardlistid | number | req | → [TimeStandardList](#timestandardlist).timestandardlistid |
| marker | string | opt | flags missed standard / fulfilled qualification |
| FEE | [Fee](#fee) | opt | fine for missed standard |

## Judge

| Field | Type | Req | Notes |
|---|---|---|---|
| officialid | number | req | → [Official](#official).officialid |
| number | number | opt | e.g. lane number for timekeepers |
| role | enum\<OTH\|MDR\|TDG\|REF\|STA\|ANN\|JOS\|CTIK\|TIK\|CFIN\|FIN\|CIOT\|IOT\|FSR\|COC\|CREC\|REC\|CRS\|CR\|MED\> | opt | default OTH |

## Official

| Field | Type | Req | Notes |
|---|---|---|---|
| officialid | number | req | unique per meet; required if referenced by Judge |
| firstname | string | req | |
| lastname | string | req | |
| nameprefix | string | opt | |
| gender | enum\<M\|F\> | opt | |
| nation | enum\<NationCode\> | opt | |
| license | string | opt | |
| grade | string | opt | federation-specific |
| passport | string | opt | |
| CONTACT | [Contact](#contact) | opt | |

## Club

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | req | |
| name.en | string-intl | opt | |
| shortname | string | opt | ≤20 chars |
| shortname.en | string-intl | opt | |
| code | string | opt | official federation club code |
| region | string | opt | |
| nation | enum\<NationCode\> | opt | |
| type | enum\<CLUB\|NATIONALTEAM\|REGIONALTEAM\|UNATTACHED\> | opt | default CLUB |
| number | number | opt | not in recordlist context |
| swrid | number | opt | swimrankings.net club id |
| CONTACT | [Contact](#contact) | opt | not in recordlist context |
| ATHLETES | [Athlete](#athlete)[] | opt[] | not in recordlist context |
| RELAYS | [Relay](#relay)[] | opt[] | not in recordlist context |
| OFFICIALS | [Official](#official)[] | opt[] | not in recordlist context |

## Athlete

| Field | Type | Req | Notes |
|---|---|---|---|
| athleteid | number | req | unique per meet |
| firstname | string | req | |
| firstname.en | string-intl | opt | |
| lastname | string | req | |
| lastname.en | string-intl | opt | |
| nameprefix | string | opt | |
| birthdate | date | req | if only birth year known, use Jan 1 |
| gender | enum\<M\|F\> | req | |
| nation | enum\<NationCode\> | opt | |
| level | string | opt | pairs with Agegroup.levels |
| license | string | opt | meet-only |
| passport | string | opt | |
| swrid | number | opt | swimrankings.net athlete id |
| CLUB | [Club](#club) | opt | recordlist-only |
| HANDICAP | [Handicap](#handicap) | opt | |
| ENTRIES | [Entry](#entry)[] | opt[] | meet-only |
| RESULTS | [Result](#result)[] | opt[] | meet-only |

## Handicap

| Field | Type | Req | Notes |
|---|---|---|---|
| breast | enum\<0-15\|GER.AB\|GER.GB\> | req | |
| free | enum\<0-15\|GER.AB\|GER.GB\> | req | freestyle/back/fly class |
| medley | enum\<0-15\|GER.AB\|GER.GB\> | req | |
| exception | string | opt | |

## Entry

| Field | Type | Req | Notes |
|---|---|---|---|
| eventid | number | req | → [Event](#event).eventid |
| heatid | number | opt | → [Heat](#heat).heatid |
| lane | number | opt | (eventid, heatid, lane) unique per meet |
| agegroupid | number | opt | → [Agegroup](#agegroup).agegroupid |
| entrytime | swimtime | opt | |
| entrycourse | enum\<CourseCode\> | opt | pool length for entry time |
| status | enum\<EXH\|RJC\|SICK\|WDR\> | opt | empty = regular entry |
| MEETINFO | [MeetInfo](#meetinfo) | opt | qualification-result info |
| RELAYPOSITIONS | [RelayPosition](#relayposition)[] | opt[] | relay entries only |

## MeetInfo

| Field | Type | Req | Notes |
|---|---|---|---|
| city | string | req(Record)/opt | required under Record |
| date | date | req(Record)/opt | required under Record |
| nation | enum\<NationCode\> | req(Record)/opt | required under Record |
| daytime | daytime | opt | |
| name | string | opt | |
| state | string | opt | |
| course | enum\<CourseCode\> | opt | Entry/RelayPosition context |
| approved | string | opt | Entry/RelayPosition context; empty = not approved |
| qualificationtime | swimtime | opt | Entry/RelayPosition context; default = entrytime |
| POOL | [Pool](#pool) | opt | |

## Result

| Field | Type | Req | Notes |
|---|---|---|---|
| resultid | number | req | unique per meet |
| eventid | number | req | → [Event](#event).eventid |
| heatid | number | opt | → [Heat](#heat).heatid |
| lane | number | opt | |
| swimtime | swimtime | req | |
| status | enum\<EXH\|DSQ\|DNS\|DNF\|SICK\|WDR\> | opt | empty = regular result |
| points | number | opt | per meet's scoring table |
| reactiontime | reactiontime | opt | relay: first swimmer's |
| comment | string | opt | |
| SPLITS | [Split](#split)[] | opt[] | continuous |
| RELAYPOSITIONS | [RelayPosition](#relayposition)[] | opt[] | relay results only |

## Split

| Field | Type | Req | Notes |
|---|---|---|---|
| distance | number | req | |
| swimtime | swimtime | req | continuous |

## Relay

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | opt | |
| gender | enum\<M\|F\|X\> | req | meet context |
| number | number | opt | meet context; distinguishes same-club teams |
| handicap | enum\<0\|14\|20\|34\|49\> | opt | meet context; default 0 |
| agemin | number | req | relay-record context; -1=none |
| agemax | number | req | relay-record context; -1=none |
| agetotalmin | number | req | relay-record context; -1=unknown |
| agetotalmax | number | req | relay-record context; -1=unknown |
| CLUB | [Club](#club) | opt | record context |
| RELAYPOSITIONS | [RelayPosition](#relayposition)[] | opt[] | record context |
| ENTRIES | [Entry](#entry)[] | opt[] | meet context |
| RESULTS | [Result](#result)[] | opt[] | meet context |

## RelayPosition

| Field | Type | Req | Notes |
|---|---|---|---|
| number | number | req | 1-based; -1=reserve swimmer |
| athleteid | number | opt | → [Athlete](#athlete).athleteid; meet context |
| ATHLETE | [Athlete](#athlete) | opt/req(Record) | record context; required there |
| reactiontime | reactiontime | opt | first swimmer=start, others=take-over |
| status | enum\<DSQ\|DNF\> | opt | absent = finished correctly |
| MEETINFO | [MeetInfo](#meetinfo) | opt | relay-entry context only |

## RecordList

| Field | Type | Req | Notes |
|---|---|---|---|
| name | string | req | |
| course | enum\<CourseCode\> | req | |
| gender | enum\<M\|F\|X\> | req | |
| type | enum\<WR\|OR\|ER\|PAR\|AFR\|AR\|OCR\|CWR\|NationCode\|"Nation.Region"\> | opt | extendable, prefix `NATION.` |
| nation | string | opt | national/regional only; empty=international |
| region | string | opt | requires nation set |
| handicap | enum\<1-15\|20\|34\|49\> | opt | |
| order | number | opt | |
| updated | date | opt | |
| AGEGROUP | [Agegroup](#agegroup) | opt | default Open |
| RECORDS | [Record](#record)[] | req[] | |

## Record

| Field | Type | Req | Notes |
|---|---|---|---|
| swimtime | swimtime | req | |
| status | string | opt | e.g. "Ratification pending" |
| comment | string | opt | |
| SWIMSTYLE | [SwimStyle](#swimstyle) | req | |
| ATHLETE | [Athlete](#athlete) | opt | individual records only |
| RELAY | [Relay](#relay) | opt | relay records only; both absent ⇒ "record standard time" |
| MEETINFO | [MeetInfo](#meetinfo) | opt | |
| SPLITS | [Split](#split)[] | opt[] | |

## TimeStandardList

| Field | Type | Req | Notes |
|---|---|---|---|
| timestandardlistid | number | req | |
| name | string | req | |
| course | enum\<CourseCode\> | req | |
| gender | enum\<M\|F\|X\> | req | |
| type | enum\<DEFAULT\|MAXIMUM\|MINIMUM\> | opt | default MAXIMUM |
| handicap | enum\<1-15\|20\|34\|49\> | opt | |
| AGEGROUP | [Agegroup](#agegroup) | opt | default Open |
| TIMESTANDARDS | [TimeStandard](#timestandard)[] | req[] | |

## TimeStandard

| Field | Type | Req | Notes |
|---|---|---|---|
| swimtime | swimtime | req | |
| SWIMSTYLE | [SwimStyle](#swimstyle) | req | unique within parent TIMESTANDARDS |

## Enum code sources

| Enum | Source |
|---|---|
| NationCode | FINA 3-letter codes — `http://www.swimrankings.net/files/Lenex_Nation.txt` |
| CountryCode | ISO 2-letter postal codes — `http://www.swimrankings.net/files/Lenex_Country.txt` |
| CurrencyCode | ISO 3-letter codes — `http://www.swimrankings.net/files/Lenex_Currency.txt` |
| CourseCode | `LCM \| SCM \| SCY \| SCM16 \| SCM20 \| SCM33 \| SCY20 \| SCY27 \| SCY33 \| SCY36 \| OPEN` |
| PointTable ids | `http://www.swimrankings.net/files/Lenex_PointTable.txt` |

## Global constraints

| Rule |
|---|
| Every element/collection may carry a `[name]id` attribute (e.g. `athleteid`); unique across all instances of that element type per meet. |
| `Xid` fields reference the matching `Xid` attribute on the target element, same-meet scope unless noted. |
| Root element: `LENEX`. File extension `.lef`; zipped form `.lxf`. |
