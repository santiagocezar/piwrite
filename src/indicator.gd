extends Node2D

export var radius = 5.0
export (int, "Left", "Right") var axis

func _ready():
	pass # Replace with function body.

var pos: Vector2

func _process(delta):
	var x
	var y
	if axis == 0:
		x = Input.get_joy_axis(0, JOY_ANALOG_LX)
		y = Input.get_joy_axis(0, JOY_ANALOG_LY)
	else:
		x = Input.get_joy_axis(0, JOY_ANALOG_RX)
		y = Input.get_joy_axis(0, JOY_ANALOG_RY)
	
	pos.x = x * sqrt(1 - .5 * (y * y)) * radius
	pos.y = y * sqrt(1 - .5 * (x * x)) * radius
	$Trail.add_point(pos)
	update()
	while $Trail.get_point_count() > 5:
		$Trail.remove_point(0)

func _draw():
	draw_circle(pos, 5, Color.black)

