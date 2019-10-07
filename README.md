# Service :: TargetDicts

[TOC]

## Предназначение

Сервис для получения и хранения словарей MyTarget.  

### Общие словари
- Мобильные операторы (mobile_operators)
- Вендоры (vendors)
- Интересы (interests)
- Регионы (regions)

### Клиентские словари
- Пакеты (packages)
- Сегменты (segments)

## Методы

### GET /account/{account}/dicts/{dict1,dict2,...}

Метод возвращает клиентские словари из хранилища  

Пример ответа:  
```json
{
    "result": [
        {
            "dict1": [
                {
                    "name": "Dict 1 item 1",
                    "id": 123
                },
                {
                    "name": "Dict 1 item 2",
                    "id": 124
                }
            ],
            "dict2": [
                {
                    "name": "Dict 2 item 1",
                    "id": 123
                },
                {
                    "name": "Dict 2 item 2",
                    "id": 124
                }
            ]
        }
    ]
}
```

### GET /dicts/{dict1,dict2,...}

Метод возвращает общие словари из хранилища  

Ответ см. выше


### PUT /dicts/account/{account}/update

Метод запрашивает все клиентские словари в MyTarget и сохраняет их в хранилище.  


### PUT /dicts/update

Метод запрашивает все общие словари в MyTarget и сохраняет их в хранилище.  
Этот метод должен вызываться периодически, например, раз в сутки


## Конфигурация

```json
{
    "client_dicts": [
        {"name": "counters", "url": "v2/remarketing/counters.json", "fields": "id,name,description"},
        {"name": "users_lists", "url": "v2/remarketing/users_lists.json"}
    ],
    "common_dicts": [
        {"name": "mobile_operators", "url": "v2/mobile_operators.json"},
        {"name": "operation_systems", "url": "v2/mobile_os.json"}
    ]
}