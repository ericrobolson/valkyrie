defmodule Parse do
  def tokenize(str) do
    str
    |> String.replace("(", " ( ")
    |> String.replace(")", " ) ")
    |> String.split()
  end

  def parse(program) do
    program |> tokenize |> parse([]) |> hd
  end

  def parse(["(" | tail], acc) do
    {rem_tokens, sub_tree} = parse(tail, [])
    parse(rem_tokens, [sub_tree | acc])
  end

  def parse([")" | tail], acc) do
    {tail, Enum.reverse(acc)}
  end

  def parse([], acc) do
    Enum.reverse(acc)
  end

  def parse([head | tail], acc) do
    parse(tail, [atom(head) | acc])
  end

  def atom(token) do
    case Integer.parse(token) do
      {value, ""} ->
        value

      :error ->
        case Float.parse(token) do
          {value, ""} ->
            value

          :error ->
            String.to_atom(token)
        end
    end
  end
end

defmodule Env do
  def new_env(outer \\ nil) do
    env = %{
      :+ => &(List.first(&1) + List.last(&1)),
      :- => &(List.first(&1) - List.last(&1)),
      :* => &(List.first(&1) * List.last(&1)),
      :/ => &(List.first(&1) / List.last(&1)),
      :< => &(List.first(&1) < List.last(&1)),
      :> => &(List.first(&1) > List.last(&1)),
      :<= => &(List.first(&1) <= List.last(&1)),
      :>= => &(List.first(&1) >= List.last(&1)),
      := => &(List.first(&1) == List.last(&1)),
      :pi => :math.pi(),
      :acosh => &:math.acosh(List.first(&1)),
      :asin => &:math.asin(List.first(&1)),
      :asinh => &:math.asinh(List.first(&1)),
      :atan => &:math.atan(List.first(&1)),
      :atan2 => &:math.atan2(List.last(&1), List.first(&1)),
      :atanh => &:math.atanh(List.first(&1)),
      :ceil => &:math.ceil(List.first(&1)),
      :cos => &:math.cos(List.first(&1)),
      :cosh => &:math.cosh(List.first(&1)),
      :exp => &:math.exp(List.first(&1)),
      :floor => &:math.floor(List.first(&1)),
      :fmod => &:math.fmod(List.first(&1), List.last(&1)),
      :log => &:math.log(List.first(&1)),
      :log10 => &:math.log10(List.first(&1)),
      :log2 => &:math.log2(List.first(&1)),
      :pow => &:math.pow(List.first(&1), List.last(&1)),
      :sin => &:math.sin(List.first(&1)),
      :sinh => &:math.sinh(List.first(&1)),
      :sqrt => &:math.sqrt(List.first(&1)),
      :tan => &:math.tan(List.first(&1)),
      :tanh => &:math.tanh(List.first(&1)),
      :car => &List.first(List.last(&1)),
      :cdr => &tl(List.last(&1)),
      :cons => &([List.first(&1)] ++ List.last(&1)),
      :begin => &List.last(&1),
      :max => &max(List.first(&1), List.last(&1)),
      :min => &min(List.first(&1), List.last(&1)),
      :and => &(List.first(&1) and List.last(&1)),
      :or => &(List.first(&1) or List.last(&1)),
      :not => &(not List.first(&1)),
      :null? => &(List.first(&1) == []),
      :number? => &is_number(List.first(&1)),
      :list => & &1,
      :list? => &is_list(&1),
      :symbol? => &is_bitstring(List.first(&1)),
      :apply => & &1.(&2),
      :append => &(&1 ++ &2),
      :procedure? => &is_function(&1)
    }

    case outer do
      nil -> env
      _ -> Map.put(%{}, :outer, outer)
    end
  end

  def put(k, v, env) do
    Map.put(env, k, v)
  end

  def get(k, env) do
    case [Map.get(env, k), Map.get(env, :outer)] do
      [nil, nil] -> nil
      [nil, outer_env] -> get(k, outer_env)
      [val, _] -> val
    end
  end
end

defmodule Eval do
  def eval(x, env, _) do
    case env do
      nil -> eval(x, Env.new_env())
      _ -> eval(x, env)
    end
  end

  defp eval(x, env) when is_atom(x) do
    {Env.get(x, env), env}
  end

  defp eval(x, env) when is_number(x) do
    {x, env}
  end

  defp eval([:if, test, conseq | alt], env) do
    alt = sanitize(alt)

    case eval(test, env) do
      {true, _} -> eval(conseq, env)
      {false, _} -> eval(alt, env)
    end
  end

  defp eval([:define, symbol | exp], env) do
    exp = sanitize(exp)
    {nil, Env.put(symbol, eval(exp, env) |> elem(0), env)}
  end

  defp eval([:set!, symbol | exp], env) do
    exp = sanitize(exp)

    case Env.get(symbol, env) do
      nil -> raise "#{symbol} not defined."
      _ -> {nil, Env.put(symbol, eval(exp, env) |> elem(0), env)}
    end
  end

  defp eval(x, env) when is_list(x) do
    proc = eval(hd(x), env) |> elem(0)
    [_ | exp] = x
    parent_env = env
    child_env = Env.new_env(env)
    args = compute_args(exp, child_env) |> Enum.into([], fn x -> elem(x, 0) end)
    {proc.(args), parent_env}
  end

  defp compute_args([], _) do
    []
  end

  defp compute_args([h | t], env) do
    {result, env} = eval(h, env)
    [{result, env} | compute_args(t, env)]
  end

  defp sanitize(x) do
    if length(x) <= 1 do
      hd(x)
    else
      x
    end
  end
end

defmodule Lispex do
  def interpret(program, env) do
    program |> Parse.parse() |> Eval.eval(env, 0)
  end

  def scheme_string(exp) do
    case is_list(exp) do
      true -> "(" <> (Enum.map(exp, fn x -> scheme_string(x) end) |> Enum.join(" ")) <> ")"
      false -> to_string(exp)
    end
  end

  def repl(env \\ nil) do
    program = IO.gets("lispex> ")
    {result, env} = program |> interpret(env)
    result |> scheme_string |> IO.puts()
    repl(env)
  end
end

Lispex.repl()
