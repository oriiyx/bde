<?php
/**
* {{ query.description }}
{% for param in query.params %}
* @param {{ param.php_type.docblock_type }} ${{ param.name }}
{% endfor %}
* @return {{ query.return_type }}
*/
public function {{ query.name }}({% for param in query.params %}{{ param.php_type.type_hint }} ${{ param.name }}{% if not loop.last %}, {% endif %}{% endfor %}): {{ query.return_type_hint }}
{
    $stmt = $this->pdo->prepare("{{ query.sql }}");
    $stmt->execute([{% for param in query.params %}${{ param.name }}{% if not loop.last %}, {% endif %}{% endfor %}]);

    {% if query.returns == "one" %}
    $row = $stmt->fetch(\PDO::FETCH_ASSOC);

    if (!$row) {
        return null;
    }

    $result = new {{ query.entity_class }}();

    {% for column in query.result_columns %}
    $result->{{ column.name }} = {% if column.php_type.php_type.simple_type == "int" %}(int){% endif %}{% if column.php_type.php_type.simple_type == "float" %}(float){% endif %}$row['{{ column.name }}'];
    {% endfor %}

    return $result;

    {% elseif query.returns == "many" %}
    $rows = $stmt->fetchAll(\PDO::FETCH_ASSOC);
    $result = [];

    foreach ($rows as $row) {
        $item = new {{ query.entity_class }}();

        {% for column in query.result_columns %}
        $item->{{ column.name }} = {% if column.php_type.php_type.simple_type == "int" %}(int){% endif %}{% if column.php_type.php_type.simple_type == "float" %}(float){% endif %}$row['{{ column.name }}'];
        {% endfor %}

        $result[] = $item;
    }

    return $result;
    {% elseif query.returns == "affected" %}

    return $stmt->rowCount();
    {% endif %}
}