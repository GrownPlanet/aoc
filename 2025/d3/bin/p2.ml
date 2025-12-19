let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let int_of_char ch = Char.code ch - Char.code '0'
let length = 12 (* can solve part 1 if changed to `2` *)
let rec pow a n = if n = 0 then 1 else a * pow a (n - 1)

let rec int_of_list list =
  match list with
  | a :: rest -> (a * pow 10 (List.length list - 1)) + int_of_list rest
  | _ -> 0

let rec update nums new_nums =
  match (nums, new_nums) with
  | a :: rn, b :: rnn ->
      let int_b = int_of_char b in
      if int_b > a then int_b :: List.map int_of_char rnn
      else a :: update rn rnn
  | _ -> nums

let rec find_largest largest nums =
  if List.length nums < length then largest
  else
    let new_largest = update largest (List.take length nums) in
    find_largest new_largest (List.tl nums)

let solve bank =
  let bank_list = List.init (String.length bank) (String.get bank) in
  let output =
    find_largest (List.init length (fun _ -> 0)) bank_list
  in
  let res = int_of_list output in
  res

let () =
  read_input "input.txt" |> List.map solve |> List.fold_left ( + ) 0
  |> Printf.printf "%d\n"
