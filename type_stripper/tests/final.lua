type x = {x: number}
type y = {y: number}
type z = {z: number}

type Vector2 = x & y
type Vector3 = x & y & z

local vec2: Vector2 = {x = 157, y = 157}
local vec3: Vector3 = {x = 157, y = 157, z = 157}
local illegalvec3: Vector3 = {
	x = 157,
	y = "This is a string, but the type y which Vector3 is intersecting with needs to be a number",
	true -- Same here, it needs to be a number, not a boolean
}

-- The arguments are between <>, separated by comma

-- As much keys and values as you want:
type defineTheTableTypeYourself<keyType, valueType> = {
	
	-- We gave string and number, so the key can be a string and the value a number or string
	[ keyType ] : valueType | keyType
	
}

local a: defineTheTableTypeYourself<string, number> = {
	
	funny = 6,
	number = 9,
	[1] = "The key is a number! Type error!"
	
}

-- Predefined table

type predefinedTable<typeOfValue> = {
	
	thisIsGodTier : {
		
		val1: typeOfValue,
		val2: typeOfValue,
		
	},
	
	reallyCool: typeOfValue
	
}

local tab: predefinedTable<string> = {
	
	thisIsGodTier = {
		
		val1 = "Bery gud",
		val2 = "Bery noice"
		
	},
	
	oopsWrongKeyNameAndType = true
	
}

-- type typeName = { keyName : typeName }

type tab = {
	
	-- If no key name is specified, the key is a number, else, a string
	firstSubTable : {
		
		num1: number,
		num2: number,
		bol1: boolean,
		bol2: boolean,
		
	},
	
	secondSubTable : {
		
		something: boolean?,
		funnyName: number | string
		
	},
	
	aCertainValue: boolean | number
	
}

-- The table has to STRICTLY follow the names of keys (and types of course)

local sumTable: tab = {
	
	firstSubTable = {

		num1 = 1,
		num2 = 2,
		bol1 = true,
		bol2 = false,

	},

	secondSubTable = {

		something = nil,
		funnyName = "very funny indeed",

	},

	aCertainValue = "haha string go brrr" -- Nooo!!1!  You can't just make it a string!!!!1 It's supposed to be a boolean or number!!!!!11
	
}

type num = number
type str = string
type bol = boolean
type nl = nil

local function compareNumber(num1: num, num2: num, maybeNum3: num | nl): bol

	local message: str = "OwO what's this?"
	return num1 == num2

end

-- We can use the Player object/class as the type for the player parameter
game.Players.PlayerAdded:Connect(function(player: Player)
	
	print(player.UserI) -- Key UserI not found in class "Player"!
	print(player.UserId)
	
end)

local stuffToBuy = {
	
	coolThing = 100,
	
}

-- Player class/object for player parameter, string for nameOfThing parameter
Instance.new("RemoteEvent").OnServerEvent:Connect(function(player: Player, nameOfThing: string)
	
	
	if not stuffToBuy[nameOfThing] then return end
	if player.cash.Value < stuffToBuy[nameOfThing] then return end
	
	-- Just an example
	player.cash.Value -= stuffToBuy[nameOfThing]
	player.inventory[nameOfThing].Value = true
	
end)

-- Assume this folder is full of Part's
local parts = game.Workspace.partsFolder:GetChildren()

for partIndex: number, part: BasePart in pairs(parts) do

	print(partIndex.." "..part.Name)

	-- 1 Part
	-- 2 Part
	-- 3 Part
	-- etc.
end

-- What about a dictionary with key strings and numbers?

local tab = {
	of = "i",
	[1] = 751,
	course = "love",
	[2] = 157,
	i = "type",
	[3] = "checking",
	_do = "!"	
}

-- This stuff is nice to look at
for key: string | number, value: string | number in pairs(tab) do
	
	print(key.." "..value)
	
	-- of i
	-- 1 751
	-- course love
	-- 2 157
	-- i type
	-- 3 checking
	-- _do !
	
end

-- Never do this in a real game, be smart and assign "stringToBeConverted" to the string type!
-- The assertion at return is needed though

local function canToNumber(stringToBeConverted: any): boolean | number

	-- tonumber() returns nil if it isn't be converted to a number, else, returns the number.
	
	-- You saw how we assigned any to stringToBeConverted? We can use
	-- the assertion operator to make Luau not scream about it
	-- number? as it can be a number or nil
	local result: number? = tonumber( (stringToBeConverted :: string) )
	
	if not result then return false end
	
	-- Asserted as it's gonna scream "boolean | number cannot be converted to number | nil (number?)"
	return ( result :: number)
end

canToNumber("157") -- 157!
canToNumber("am i funny? answer true pls") -- False!

-- expression( (variable :: type) )

-- If we assign type any to it, the type checker's not gonna know it is a number
local x: any = 1

-- So it's gonna scream about this, because it doesn't know if x is a number
print(x + 1)

-- Not if we assert it is a number, then it's going to be gud
print( (x :: number) + 1 )

-- local type: type?

local probablyABoolean: boolean? = true
probablyABoolean = nil

local probablyAString: string? = nil
probablyAString = ""

local probablyNil: nil? = nil -- This is useless, but by logic it is valid

local numberBooleanOrNil: number | boolean? = nil
numberBooleanOrNil = 157
numberBooleanOrNil = true

local probablyANumber: number? = 157
probablyANumber = nil
probablyANumber = true -- Type error, only accepts a number or nil!

-- local variable: type | type | type ...

local typeChecking: string | number = "Can be either a string or number!"
typeChecking = 157

local typeChecking: boolean | nil = true
typeChecking = nil

local typeChecking: number | string | boolean = "This can be a number, string or boolean!"
typeChecking = 157
typeChecking = false

local typeChecking: nil | string | boolean | number = nil
typeChecking = "This can be nil, a string, a boolean or a number!"
typeChecking = 157
typeChecking = true

local typeChecking: boolean | number | nil = "This gives a type error though, because string wasn't noted"

local datatypeTypeCheck: BrickColor

datatypeTypeCheck = BrickColor.Red() -- Valid type
datatypeTypeCheck = Vector3.new() -- Since when is Vector3 a Brickcolor?

local enumTypeCheck: Enum.UserInputType
enumTypeCheck = Enum.UserInputType.MouseButton1 -- Cool! That's a valid type

enumTypeCheck = Enum.OverrideMouseIconBehavior.ForceHide -- Not valid D:

local numberTypeChecking: number = 1
local stringTypeChecking: string = "Stop right there!!"
local booleanTypeChecking: boolean = true