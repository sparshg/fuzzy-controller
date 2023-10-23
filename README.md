# Fuzzy Drone Controller

## [Try on Web](https://sparshg.github.io/fuzzy-controller/)


<img alt="fuzzy" src="https://github.com/sparshg/fuzzy-controller/assets/43041139/9cff3b79-e547-4152-8add-93db8e69804b">

## Notation Used

I have used these symbols throughout for the controllers, more information in the next section:

Controller 1 (Contols `amplitude`):

<table>
<tr><th>Inputs</th><th>Outputs</th></tr>
<tr><td>

|  |            |
| :----: | :--------: |
| Y      | Y Position Error|
| Vy     | Y Velocity |

</td><td>
  
|  |            |
| :----: | :--------: |
|Z| Zero thrust amplitude|
|S| Small thrust amplitude|
|L| Large thrust amplitude|

</td></tr> </table>

Controller 2 (Contols `diff`)

<table>
<tr><th>Inputs</th><th>Outputs</th></tr>
<tr><td>
  
|  |            |
| :----: | :--------: |
|X| X Position Error|
|Vx| X Velocity|
|Th| Angle from horizontal|
|W| Angular velocity|

</td><td>

|  |            |
| :----: | :--------: |
| Z| Zero|
| NS/PS| Small (-/+) thrust difference|
| NM/PM| Medium (-/+) thrust difference|
| NL/PL| Large (-/+) thrust difference|

</td></tr> </table>


where Positive is the anti-clockwise direction
and Negative is the clockwise direction

## Working

This is a controller that uses [Mamdani Fuzzy inference](https://in.mathworks.com/help/fuzzy/types-of-fuzzy-inference-systems.html) system to control a drone, simulated using a physics engine.

The drone has 2 thrusters controlled by 2 fuzzy controllers. The controllers analyze the current state of the system and output the required thrust for each thruster. One takes the `(y_pos, y_vel)` and computes an `amplitude`. The other one takes `(x_pos, x_vel, angle, ang_vel)` and computes a `diff`. Then the thrusters are assigned the forces given by `amplitude + diff` and `amplitude - diff`.

A fuzzy controller is a rule-based system that uses fuzzy logic to map inputs to outputs. We can often derive these rules by observing the system and using our intuition. For example, 
```
if x_vel is positive: apply small negative force
if x_vel is negative: apply small positive force
```
We can further use and, or, not operators on fuzzy sets to create complex rules. To do this I overloaded the bitwise operators on the inputs. This creates a parse tree of the rules, which can be evaluated at the runtime to get the outputs.

To use these abstract or "fuzzy" rules on exact or "crisp" values of inputs, we fuzzify the inputs. For example, a tilt of `1 rad` to the left can be defined as `80% positive` and `20% negative`, according to whatever membership functions we define. We can then apply these fuzzy rules to the fuzzy inputs to get fuzzy outputs. Finally, we defuzzify the fuzzy outputs to get crisp outputs.
## Build and Run

Install [Rust](https://www.rust-lang.org/tools/install), and then:
```
cargo run --release
```
