let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let int_of_char ch = Char.code ch - Char.code '0'

let rec find_largest first second nums =
  match nums with
  | a :: b :: rest ->
      let int_b = int_of_char b in
      let new_first = max first (int_of_char a) in
      let new_second = if first <> new_first then int_b else max second int_b in
      find_largest new_first new_second (b :: rest)
  | [ b ] -> (first, max second (int_of_char b))
  | [] -> (first, second)

let solve bank =
  let a, b =
    List.init (String.length bank) (String.get bank)
    |> find_largest 0 0
  in
  (a * 10) + b

let () =
  read_input "input.txt" |> List.map solve |> List.fold_left ( + ) 0
  |> Printf.printf "%d\n"
