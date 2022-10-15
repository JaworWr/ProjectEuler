module solution
    implicit none
    integer, parameter :: N_COINS = 8, AMOUNT = 200
    integer, dimension(N_COINS), parameter :: coins = [1, 2, 5, 10, 20, 50, 100, 200]

contains
    function ways()
        integer :: ways

        integer, dimension(N_COINS + 1, AMOUNT + 1) :: dp
        integer :: i, j, c

        dp = 0
        dp(:, 1) = 1
        do i = 2, N_COINS + 1
            do j = 2, AMOUNT + 1
                dp(i, j) = dp(i - 1, j)
                c = coins(i - 1)
                if (j - c >= 1) then
                    dp(i, j) = dp(i, j) + dp(i, j - c)
                end if
            end do
        end do
        ways = dp(N_COINS + 1, AMOUNT + 1)

    end function ways
end module solution


program p31
    use solution
    implicit none

    print *, coins
    print *, ways()
    
end program p31