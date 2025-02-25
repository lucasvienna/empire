CREATE VIEW resource_generation AS
SELECT ub.user_id,
       SUM(br.population) as population,
       SUM(br.food)       as food,
       SUM(br.wood)       as wood,
       SUM(br.stone)      as stone,
       SUM(br.gold)       as gold
FROM user_buildings ub
         LEFT JOIN public.building_resources br
                   ON ub.building_id = br.building_id
GROUP BY ub.user_id;
