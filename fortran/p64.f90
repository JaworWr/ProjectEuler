module alg_pair
    implicit none
    public

    type :: t_alg_pair
    ! (sqrt(a) + b) / c
        integer :: a
        integer :: b
        integer :: c
    end type

    interface operator(==)
        module procedure alg_equal
    end interface

    integer, parameter :: M = 100000

contains
pure function sub_reciprocal(p, x) result(r)
    ! calculates 1 / (p - x)
    type(t_alg_pair), intent(in) :: p
    integer, intent(in) :: x
    type(t_alg_pair) :: r
    integer :: denom, m

    r%a = p%a
    r%b = p%c * x - p%b
    denom = r%a - r%b * r%b
    m = mod(denom, p%c)
    if (m /= 0) error stop "not divisible"
    r%c = denom / p%c    
end function

pure function alg_equal(p1, p2)
    type(t_alg_pair), intent(in) :: p1, p2
    logical :: alg_equal

    alg_equal = p1%a == p2%a .and. p1%b == p2%b .and. p1%c == p2%c
end function

pure function int_part(p)
    type(t_alg_pair), intent(in) :: p
    integer :: int_part
    real(kind=8) :: x

    x = sqrt(real(p%a))
    x = (x + p%b) / p%c
    int_part = int(x)
end function

pure function period_len(p)
    type(t_alg_pair), value :: p
    type(t_alg_pair), dimension(:), allocatable :: h
    integer :: period_len, idx, i, x

    allocate(h(M))
    idx = 1
    h(1) = p

    do while (.true.)
        x = int_part(p)
        p = sub_reciprocal(p, x)
        do i = 1, idx
            if (h(i) == p) then
                period_len = idx - i + 1
                return
            end if
        end do
        idx = idx + 1
        h(idx) = p

        if (idx == M) error stop "out of bounds"
    end do
    

end function
end

program p64
    use alg_pair
    
    implicit none

    type(t_alg_pair) :: p
    integer :: i, l, c

    c = 0
    do i = 2, 10000
        if (is_square(i)) then
            cycle
        end if
        p%a = i
        p%b = 0
        p%c = 1
        l = period_len(p)
        if (mod(l, 2) == 1) then
            c = c + 1
        end if
    end do
    print *, c
contains
    pure function is_square(x)
        integer, intent(in) :: x
        logical :: is_square
        integer :: a
        real(kind=8) :: t

        t = real(x)
        t = sqrt(t)
        a = nint(t)
        is_square = a * a == x
    end
end