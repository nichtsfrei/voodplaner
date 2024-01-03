# voodplaner-requirements

This library is concerned about calculating the requirements of an individual.

This is needed to get an overview about the meal requirements and fullfillments.

The caculation is mainly based on information found in:

- https://www.dge.de/wissenschaft/referenzwerte-tool

This is based on [sex based defaults](https://www.dge.de/wissenschaft/referenzwerte/energie/#c1006https://www.dge.de/wissenschaft/referenzwerte/energie/#c1006):
This is using the values of PAL 1.4.

## male

| Group                 | Age Group         | Body Height (cm) | Body Weight (kg) | Energy |
|-----------------------|-------------------|------------------|------------------|--------|
| Infants               | 0 - 4 m           | 58.4             | 5.6              | 550    |
| Infants               | 4 - 12 m          | 70.6             | 8.6              | 700    |
| Children and Teenagers| 1 - 4 y           | 92.9             | 13.9             | 1200   |
| Children and Teenagers| 4 - 7 y           | 114.5            | 20.2             | 1400   |
| Children and Teenagers| 7 - 10 y          | 133.6            | 29.3             | 1700   |
| Children and Teenagers| 10 - 13 y         | 149.4            | 41.0             | 1900   |
| Children and Teenagers| 13 - 15 y         | 166.9            | 55.5             | 2300   |
| Children and Teenagers| 15 - 19 y         | 178.2            | 69.2             | 2600   |
| Adults                | 19 - 25 y         | 179.4            | 70.8             | 2400   |
| Adults                | 25 - 51 y         | 179.2            | 70.7             | 2300   |
| Adults                | 51 - 65 y         | 176.7            | 68.7             | 2200   |
| Adults                | 65 y and older    | 174.2            | 66.8             | 2100   |


## female

| Group                 | Age Group         | Body Height (cm) | Body Weight (kg) | Energy |
|-----------------------|-------------------|------------------|------------------|--------|
| Infants               | 0 - 4 m           | 57.1             | 5.1              | 500    |
| Infants               | 4 - 12 m          | 68.7             | 7.9              | 600    |
| Children and Teenagers| 1 - 4 y           | 91.3             | 13.2             | 1100   |
| Children and Teenagers| 4 - 7 y           | 114.3            | 20.1             | 1300   |
| Children and Teenagers| 7 - 10 y          | 132.4            | 28.7             | 1500   |
| Children and Teenagers| 10 - 13 y         | 151.0            | 42.1             | 1700   |
| Children and Teenagers| 13 - 15 y         | 162.7            | 54.0             | 1900   |
| Children and Teenagers| 15 - 19 y         | 165.5            | 59.5             | 2000   |
| Adults                | 19 - 25 y         | 165.8            | 60.5             | 1900   |
| Adults                | 25 - 51 y         | 165.1            | 60.0             | 1800   |
| Adults                | 51 - 65 y         | 162.6            | 58.2             | 1700   |
| Adults                | 65 y and older    | 161.1            | 57.1             | 1700   |

When we normalize the energy to a fictional being of 100cm and 1kg and a PAL of 1.0 we use the formular:

`energy * (1/height) * (1/weight) / 1.4`

except for infants there was no PAL added so it is

`energy * (1/height) * (1/weight)`

we get the energy lookup table:

### Male

| Group                 | Age Group         | Energy |
|-----------------------|-------------------|------|
| Infants               | 0 - 4 m           | 1.66 |
| Infants               | 4 - 12 m          | 1.15 |
| Children and Teenagers| 1 - 4 y           | 0.66 |
| Children and Teenagers| 4 - 7 y           | 0.44 |
| Children and Teenagers| 7 - 10 y          | 0.31 |
| Children and Teenagers| 10 - 13 y         | 0.22 |
| Children and Teenagers| 13 - 15 y         | 0.18 |
| Children and Teenagers| 15 - 19 y         | 0.15 |
| Adults                | 19 - 25 y         | 0.14 |
| Adults                | 25 - 51 y         | 0.13 |
| Adults                | 51 - 65 y         | 0.14 |
| Adults                | 65 y and older    | 0.13 |

### Female

| Group                 | Age Group         | Energy |
|-----------------------|-------------------|------|
| Infants               | 0 - 4 m           | 1.71 |
| Infants               | 4 - 12 m          | 1.15 |
| Children and Teenagers| 1 - 4 y           | 0.65 |
| Children and Teenagers| 4 - 7 y           | 0.43 |
| Children and Teenagers| 7 - 10 y          | 0.31 |
| Children and Teenagers| 10 - 13 y         | 0.21 |
| Children and Teenagers| 13 - 15 y         | 0.16 |
| Children and Teenagers| 15 - 19 y         | 0.14 |
| Adults                | 19 - 25 y         | 0.14 |
| Adults                | 25 - 51 y         | 0.13 |
| Adults                | 51 - 65 y         | 0.14 |
| Adults                | 65 y and older    | 0.14 |

This library uses the normalized energy lookup based on age group and sex and then calculates it back to the given height and weight:

`energy * height * weight * PAL`

This library does not include a calculation for pregnancy or lacteting as the [dge](https://www.dge.de/gesunde-ernaehrung/faq/energiezufuhr/#c2971) is hinting that it is in most cases not required.

However on a rule of thumb:
- when pregnant within the second term add +250 kcal per day 
- when pregnant within the third term add 500 kcal per day 
- when lactating within the first 6 months add 500 kcal per day

The activity levels are grouped into:
- 0.95 sleeping
- 1.2 lying (sick and elderly)
- 1.4 sedentary (clerks, precision engineer)
- 1.6 moderately (students, assemblyman)
- 1.8 very (waiter, mechanic)
- 2.0 extremely (builder, athlete)

You can also calculate your own by pal by providing a set of information of a day:
Given a being sleeps for 8 hours, works for 8 hours as software engineer (1.4), does 2 hours of kick boxing (2.0) and does some tinkering with electronics for 6 hours (1.6) than you can calculate it via:

(8 * 0.95 + 8 * 1.4 + 2 * 2 + 6 * 1.6) / 24
