# Concepts
 * [Types](#types) -
 * [Inheritance](#inheritance) -
 * [Events](#events) - 
 * [Triggers](#triggers) -

## Types
```ruby

```

## Inheritance

## Events
```ruby
event DiscordEvent {
	is_bot: bool,
	message_id: MessageId,
}

query Message: DiscordEvent 
where
	is_bot,
{

}

group<Guild> {
	votes: int

	delegate log_message(msg: Message) 
	where
		!msg.is_bot,
	{
		
	}
}
```

## Triggers